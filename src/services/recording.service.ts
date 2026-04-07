import { invoke } from "@tauri-apps/api/core"

export type RecordingStatus = "idle" | "recording" | "paused"

const isTauri = (): boolean => {
  return (
    typeof window !== "undefined" &&
    ("__TAURI__" in window || "__TAURI_INTERNALS__" in window)
  )
}

export interface RecordingMetadata {
  output_path: string
  sample_rate: number
  channels: number
  started_at: string
  finished_at: string | null
}

export interface RecordingInfo {
  status: RecordingStatus
  metadata: RecordingMetadata | null
}

export const recordingService = {
  async start(outputPath: string): Promise<RecordingMetadata> {
    if (!isTauri()) {
      throw new Error("Recording is only available in the Tauri app")
    }
    return invoke<RecordingMetadata>("start_recording", {
      outputPath,
    })
  },

  async stop(): Promise<RecordingMetadata> {
    if (!isTauri()) {
      throw new Error("Recording is only available in the Tauri app")
    }
    return invoke<RecordingMetadata>("stop_recording")
  },

  async pause(): Promise<string> {
    if (!isTauri()) {
      throw new Error("Recording is only available in the Tauri app")
    }
    return invoke<string>("pause_recording")
  },

  async resume(): Promise<string> {
    if (!isTauri()) {
      throw new Error("Recording is only available in the Tauri app")
    }
    return invoke<string>("resume_recording")
  },

  async getStatus(): Promise<RecordingInfo> {
    if (!isTauri()) {
      return { status: "idle", metadata: null }
    }
    return invoke<RecordingInfo>("get_recording_status")
  },
}
