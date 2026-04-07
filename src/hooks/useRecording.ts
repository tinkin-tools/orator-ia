import { useState, useCallback, useEffect } from "react"
import {
  recordingService,
  RecordingStatus,
  RecordingMetadata,
} from "../services/recording.service"

interface UseRecordingState {
  status: RecordingStatus
  metadata: RecordingMetadata | null
  isLoading: boolean
  error: string | null
}

interface UseRecordingActions {
  start: (outputPath: string) => Promise<RecordingMetadata | null>
  stop: () => Promise<RecordingMetadata | null>
  pause: () => Promise<boolean>
  resume: () => Promise<boolean>
  refreshStatus: () => Promise<void>
  clearError: () => void
}

export type UseRecordingReturn = UseRecordingState & UseRecordingActions

export function useRecording(): UseRecordingReturn {
  const [status, setStatus] = useState<RecordingStatus>("idle")
  const [metadata, setMetadata] = useState<RecordingMetadata | null>(null)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const refreshStatus = useCallback(async () => {
    try {
      const info = await recordingService.getStatus()
      setStatus(info.status)
      setMetadata(info.metadata)
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : "Failed to get recording status"
      setError(errorMessage)
    }
  }, [])

  useEffect(() => {
    refreshStatus()
  }, [refreshStatus])

  const start = useCallback(async (outputPath: string) => {
    setIsLoading(true)
    setError(null)
    try {
      const result = await recordingService.start(outputPath)
      setStatus("recording")
      setMetadata(result)
      return result
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : "Failed to start recording"
      setError(errorMessage)
      return null
    } finally {
      setIsLoading(false)
    }
  }, [])

  const stop = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    try {
      const result = await recordingService.stop()
      setStatus("idle")
      setMetadata(null)
      return result
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : "Failed to stop recording"
      setError(errorMessage)
      return null
    } finally {
      setIsLoading(false)
    }
  }, [])

  const pause = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    try {
      await recordingService.pause()
      setStatus("paused")
      return true
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : "Failed to pause recording"
      setError(errorMessage)
      return false
    } finally {
      setIsLoading(false)
    }
  }, [])

  const resume = useCallback(async () => {
    setIsLoading(true)
    setError(null)
    try {
      await recordingService.resume()
      setStatus("recording")
      return true
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : "Failed to resume recording"
      setError(errorMessage)
      return false
    } finally {
      setIsLoading(false)
    }
  }, [])

  const clearError = useCallback(() => {
    setError(null)
  }, [])

  return {
    status,
    metadata,
    isLoading,
    error,
    start,
    stop,
    pause,
    resume,
    refreshStatus,
    clearError,
  }
}
