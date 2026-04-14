use hound;
use std::fs::File;
use std::path::Path;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::domain::audio::entities::AudioFile;
use crate::domain::audio::services::AudioProcessingService;
use crate::domain::settings::entities::AudioProcessingConfig;
use crate::domain::shared::{DomainError, DomainResult};

pub struct SymphoniaAudioProcessor;

impl AudioProcessingService for SymphoniaAudioProcessor {
    fn process_audio_file(
        &self,
        audio_file: &AudioFile,
        config: &AudioProcessingConfig,
    ) -> DomainResult<()> {
        self.process_wav_file_internal(
            audio_file.input_path_str(),
            audio_file.output_path_str(),
            config,
        )
        .map_err(|e| DomainError::ProcessingError(e.to_string()))
    }

    fn validate_audio_file(&self, input_path: &str) -> DomainResult<()> {
        if !Path::new(input_path).exists() {
            return Err(DomainError::FileSystemError(format!(
                "Input file not found: {input_path}"
            )));
        }
        Ok(())
    }
}

impl Default for SymphoniaAudioProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl SymphoniaAudioProcessor {
    pub fn new() -> Self {
        Self
    }

    fn process_wav_file_internal(
        &self,
        input_path: &str,
        output_path: &str,
        config: &AudioProcessingConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let src = File::open(input_path)?;
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = Path::new(input_path).extension() {
            if let Some(ext_str) = ext.to_str() {
                hint.with_extension(ext_str);
            }
        }

        let meta = symphonia::default::get_probe().format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;

        let mut format = meta.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or("No audio track found")?;

        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &DecoderOptions::default())?;

        let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
        let channels = track.codec_params.channels.map(|c| c.count()).unwrap_or(1);

        let all_samples = self.decode_audio(&mut *format, &mut *decoder)?;

        let processed_samples = self.remove_silence(&all_samples, sample_rate, channels, config);

        self.write_wav_file(
            output_path,
            &processed_samples,
            sample_rate,
            channels.try_into().unwrap(),
        )?;

        Ok(())
    }

    fn decode_audio(
        &self,
        format: &mut dyn FormatReader,
        decoder: &mut dyn symphonia::core::codecs::Decoder,
    ) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let mut all_samples: Vec<f32> = Vec::new();

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(Error::ResetRequired) => {
                    break;
                }
                Err(Error::IoError(err)) => {
                    if err.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    } else {
                        return Err(Box::new(err));
                    }
                }
                Err(err) => return Err(Box::new(err)),
            };

            let decoded = decoder.decode(&packet)?;
            match decoded {
                AudioBufferRef::F32(buf) => {
                    for frame in 0..buf.frames() {
                        for ch in 0..buf.spec().channels.count() {
                            all_samples.push(buf.chan(ch)[frame]);
                        }
                    }
                }
                AudioBufferRef::S16(buf) => {
                    for frame in 0..buf.frames() {
                        for ch in 0..buf.spec().channels.count() {
                            all_samples.push(buf.chan(ch)[frame] as f32 / 32768.0);
                        }
                    }
                }
                AudioBufferRef::S32(buf) => {
                    for frame in 0..buf.frames() {
                        for ch in 0..buf.spec().channels.count() {
                            all_samples.push(buf.chan(ch)[frame] as f32 / 2147483648.0);
                        }
                    }
                }
                _ => return Err("Unsupported audio format (only F32, S16, S32 supported)".into()),
            }
        }

        Ok(all_samples)
    }

    fn remove_silence(
        &self,
        samples: &[f32],
        sample_rate: u32,
        channels: usize,
        config: &AudioProcessingConfig,
    ) -> Vec<f32> {
        let min_silence_duration = config.min_silence_duration.value.as_f64().unwrap() as f32;
        let min_audio_duration = config.min_audio_duration.value.as_f64().unwrap() as f32;

        let min_silence_samples = (min_silence_duration * sample_rate as f32) as usize;
        let min_audio_samples = (min_audio_duration * sample_rate as f32) as usize;

        let mut result = Vec::new();
        let mut i = 0;

        while i < samples.len() {
            let mut start = i;
            while start < samples.len() {
                let mut frame_silent = true;
                for ch in 0..channels {
                    let sample_idx = start + ch;
                    let silence_threshold =
                        config.silence_threshold.value.as_f64().unwrap_or(0.01) as f32;
                    if sample_idx < samples.len() && samples[sample_idx].abs() >= silence_threshold
                    {
                        frame_silent = false;
                        break;
                    }
                }

                if !frame_silent {
                    break;
                }
                start += channels;
            }

            if start >= samples.len() {
                break;
            }

            let mut end = start;
            let mut silence_count = 0;

            while end < samples.len() {
                let mut frame_silent = true;
                for ch in 0..channels {
                    let sample_idx = end + ch;
                    let silence_threshold =
                        config.silence_threshold.value.as_f64().unwrap_or(0.01) as f32;
                    if sample_idx < samples.len() && samples[sample_idx].abs() >= silence_threshold
                    {
                        frame_silent = false;
                        break;
                    }
                }

                if frame_silent {
                    silence_count += 1;
                    if silence_count >= min_silence_samples {
                        break;
                    }
                } else {
                    silence_count = 0;
                }
                end += channels;
            }

            if end - start >= min_audio_samples * channels {
                result.extend_from_slice(&samples[start..end]);
            }

            i = end;
        }

        result
    }

    fn write_wav_file(
        &self,
        output_path: &str,
        samples: &[f32],
        sample_rate: u32,
        channels: u8,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let spec = hound::WavSpec {
            channels: channels as u16,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(output_path, spec)?;

        for &sample in samples {
            let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
            writer.write_sample(sample_i16)?;
        }

        writer.finalize()?;
        Ok(())
    }
}
