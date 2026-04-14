use chrono::Utc;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::domain::audio::entities::recording::{
    RecordingConfig, RecordingInfo, RecordingMetadata,
};
use crate::domain::audio::services::RecordingService;
use crate::domain::shared::{DomainError, DomainResult};


enum AudioCommand {
    Start {
        output_path: String,
        reply: Sender<DomainResult<RecordingMetadata>>,
    },
    Stop {
        reply: Sender<DomainResult<RecordingMetadata>>,
    },
    Pause {
        reply: Sender<DomainResult<()>>,
    },
    Resume {
        reply: Sender<DomainResult<()>>,
    },
    GetInfo {
        reply: Sender<RecordingInfo>,
    },
    Shutdown,
}


struct AudioThreadState {
    stream: Option<cpal::Stream>,
    writer: Option<Arc<Mutex<Option<WavWriter<std::io::BufWriter<std::fs::File>>>>>>,
    is_recording: Option<Arc<AtomicBool>>,
    is_paused: Option<Arc<AtomicBool>>,
    metadata: Option<RecordingMetadata>,
}

impl AudioThreadState {
    fn new() -> Self {
        Self {
            stream: None,
            writer: None,
            is_recording: None,
            is_paused: None,
            metadata: None,
        }
    }

    fn handle_start(&mut self, output_path: String) -> DomainResult<RecordingMetadata> {
        if self.stream.is_some() {
            return Err(DomainError::ValidationError(
                "A recording is already in progress".to_string(),
            ));
        }

        let host = cpal::default_host();
        let device = host.default_input_device().ok_or_else(|| {
            DomainError::ProcessingError("No input device (microphone) found".to_string())
        })?;

        let supported_config = device.default_input_config().map_err(|e| {
            DomainError::ProcessingError(format!("Failed to get input config: {e}"))
        })?;

        let sample_rate = supported_config.sample_rate().0;
        let channels = supported_config.channels();

        let wav_spec = WavSpec {
            channels,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let writer = WavWriter::create(&output_path, wav_spec).map_err(|e| {
            DomainError::FileSystemError(format!("Failed to create output file: {e}"))
        })?;
        let writer = Arc::new(Mutex::new(Some(writer)));

        let is_recording = Arc::new(AtomicBool::new(true));
        let is_paused = Arc::new(AtomicBool::new(false));

        let stream = Self::build_stream(
            &device,
            &supported_config,
            Arc::clone(&writer),
            Arc::clone(&is_recording),
            Arc::clone(&is_paused),
        )?;

        stream.play().map_err(|e| {
            DomainError::ProcessingError(format!("Failed to start recording: {e}"))
        })?;

        let file_path = crate::domain::shared::FilePath::new(output_path)?;
        let metadata = RecordingMetadata::new(file_path, sample_rate, channels, Utc::now());

        self.stream = Some(stream);
        self.writer = Some(writer);
        self.is_recording = Some(is_recording);
        self.is_paused = Some(is_paused);
        self.metadata = Some(metadata.clone());

        Ok(metadata)
    }

    fn handle_stop(&mut self) -> DomainResult<RecordingMetadata> {
        if self.stream.is_none() {
            return Err(DomainError::ValidationError(
                "No recording in progress".to_string(),
            ));
        }

        if let Some(is_recording) = self.is_recording.take() {
            is_recording.store(false, Ordering::Relaxed);
        }

       
        self.stream.take();
        self.is_paused.take();

        if let Some(writer) = self.writer.take() {
            if let Ok(mut guard) = writer.lock() {
                if let Some(w) = guard.take() {
                    w.finalize().map_err(|e| {
                        DomainError::ProcessingError(format!(
                            "Failed to finalize WAV file: {e}"
                        ))
                    })?;
                }
            }
        }

        let mut metadata = self
            .metadata
            .take()
            .ok_or_else(|| DomainError::ProcessingError("Missing metadata".to_string()))?;
        metadata.finalize(Utc::now());

        Ok(metadata)
    }

    fn handle_pause(&mut self) -> DomainResult<()> {
        let is_paused = self.is_paused.as_ref().ok_or_else(|| {
            DomainError::ValidationError("No recording in progress".to_string())
        })?;

        if is_paused.load(Ordering::Relaxed) {
            return Err(DomainError::ValidationError(
                "Recording is already paused".to_string(),
            ));
        }

        is_paused.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn handle_resume(&mut self) -> DomainResult<()> {
        let is_paused = self.is_paused.as_ref().ok_or_else(|| {
            DomainError::ValidationError("No recording in progress".to_string())
        })?;

        if !is_paused.load(Ordering::Relaxed) {
            return Err(DomainError::ValidationError(
                "Recording is not paused".to_string(),
            ));
        }

        is_paused.store(false, Ordering::Relaxed);
        Ok(())
    }

    fn handle_get_info(&self) -> RecordingInfo {
        match &self.metadata {
            None => RecordingInfo::idle(),
            Some(metadata) => {
                let paused = self
                    .is_paused
                    .as_ref()
                    .map(|p| p.load(Ordering::Relaxed))
                    .unwrap_or(false);

                if paused {
                    RecordingInfo::paused(metadata.clone())
                } else {
                    RecordingInfo::recording(metadata.clone())
                }
            }
        }
    }

    fn build_stream(
        device: &cpal::Device,
        config: &cpal::SupportedStreamConfig,
        writer: Arc<Mutex<Option<WavWriter<std::io::BufWriter<std::fs::File>>>>>,
        is_recording: Arc<AtomicBool>,
        is_paused: Arc<AtomicBool>,
    ) -> DomainResult<cpal::Stream> {
        let err_fn = |err: cpal::StreamError| {
            eprintln!("Recording stream error: {err}");
        };

        let stream_config: cpal::StreamConfig = config.clone().into();

        let stream = match config.sample_format() {
            cpal::SampleFormat::I16 => {
                let writer = Arc::clone(&writer);
                let is_recording = Arc::clone(&is_recording);
                let is_paused = Arc::clone(&is_paused);
                device.build_input_stream(
                    &stream_config,
                    move |data: &[i16], _: &cpal::InputCallbackInfo| {
                        if !is_recording.load(Ordering::Relaxed)
                            || is_paused.load(Ordering::Relaxed)
                        {
                            return;
                        }
                        if let Ok(mut guard) = writer.lock() {
                            if let Some(ref mut w) = *guard {
                                for &sample in data {
                                    let _ = w.write_sample(sample);
                                }
                            }
                        }
                    },
                    err_fn,
                    None,
                )
            }
            cpal::SampleFormat::F32 => {
                let writer = Arc::clone(&writer);
                let is_recording = Arc::clone(&is_recording);
                let is_paused = Arc::clone(&is_paused);
                device.build_input_stream(
                    &stream_config,
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        if !is_recording.load(Ordering::Relaxed)
                            || is_paused.load(Ordering::Relaxed)
                        {
                            return;
                        }
                        if let Ok(mut guard) = writer.lock() {
                            if let Some(ref mut w) = *guard {
                                for &sample in data {
                                    let sample_i16 =
                                        (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                                    let _ = w.write_sample(sample_i16);
                                }
                            }
                        }
                    },
                    err_fn,
                    None,
                )
            }
            cpal::SampleFormat::U16 => {
                let writer = Arc::clone(&writer);
                let is_recording = Arc::clone(&is_recording);
                let is_paused = Arc::clone(&is_paused);
                device.build_input_stream(
                    &stream_config,
                    move |data: &[u16], _: &cpal::InputCallbackInfo| {
                        if !is_recording.load(Ordering::Relaxed)
                            || is_paused.load(Ordering::Relaxed)
                        {
                            return;
                        }
                        if let Ok(mut guard) = writer.lock() {
                            if let Some(ref mut w) = *guard {
                                for &sample in data {
                                    let sample_i16 = (sample as i32 - 32768) as i16;
                                    let _ = w.write_sample(sample_i16);
                                }
                            }
                        }
                    },
                    err_fn,
                    None,
                )
            }
            fmt => {
                return Err(DomainError::ProcessingError(format!(
                    "Unsupported sample format: {fmt:?}"
                )));
            }
        }
        .map_err(|e| {
            DomainError::ProcessingError(format!("Failed to build audio stream: {e}"))
        })?;

        Ok(stream)
    }
}


pub struct CpalRecorder {
    cmd_tx: Sender<AudioCommand>,
    _thread: JoinHandle<()>,
}

impl CpalRecorder {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx): (Sender<AudioCommand>, Receiver<AudioCommand>) = mpsc::channel();

        let thread = thread::spawn(move || {
            let mut state = AudioThreadState::new();

            for cmd in cmd_rx {
                match cmd {
                    AudioCommand::Start { output_path, reply } => {
                        let _ = reply.send(state.handle_start(output_path));
                    }
                    AudioCommand::Stop { reply } => {
                        let _ = reply.send(state.handle_stop());
                    }
                    AudioCommand::Pause { reply } => {
                        let _ = reply.send(state.handle_pause());
                    }
                    AudioCommand::Resume { reply } => {
                        let _ = reply.send(state.handle_resume());
                    }
                    AudioCommand::GetInfo { reply } => {
                        let _ = reply.send(state.handle_get_info());
                    }
                    AudioCommand::Shutdown => break,
                }
            }
        });

        Self {
            cmd_tx,
            _thread: thread,
        }
    }

    fn send_and_receive<T>(&self, make_cmd: impl FnOnce(Sender<T>) -> AudioCommand) -> DomainResult<T> {
        let (reply_tx, reply_rx) = mpsc::channel();
        let cmd = make_cmd(reply_tx);

        self.cmd_tx
            .send(cmd)
            .map_err(|_| DomainError::ProcessingError("Audio thread is not running".to_string()))?;

        reply_rx
            .recv()
            .map_err(|_| DomainError::ProcessingError("Audio thread did not respond".to_string()))
    }
}

impl RecordingService for CpalRecorder {
    fn start_recording(&mut self, config: RecordingConfig) -> DomainResult<RecordingMetadata> {
        let output_path = config.output_path.as_str().to_string();
        self.send_and_receive(|reply| AudioCommand::Start { output_path, reply })?
    }

    fn stop_recording(&mut self) -> DomainResult<RecordingMetadata> {
        self.send_and_receive(|reply| AudioCommand::Stop { reply })?
    }

    fn pause_recording(&mut self) -> DomainResult<()> {
        self.send_and_receive(|reply| AudioCommand::Pause { reply })?
    }

    fn resume_recording(&mut self) -> DomainResult<()> {
        self.send_and_receive(|reply| AudioCommand::Resume { reply })?
    }

    fn get_recording_info(&self) -> RecordingInfo {
        let (reply_tx, reply_rx) = mpsc::channel();
        let cmd = AudioCommand::GetInfo { reply: reply_tx };

        if self.cmd_tx.send(cmd).is_err() {
            return RecordingInfo::idle();
        }

        reply_rx.recv().unwrap_or_else(|_| RecordingInfo::idle())
    }
}
