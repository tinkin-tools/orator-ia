import "./record.css"
import React, { useState, useEffect, useRef, useCallback } from "react"
import Modal from "../../components/ui/modal/Modal"

export interface RecordProps {}

const TOTAL_BARS = 40

const Record: React.FC<RecordProps> = ({}) => {
  const [isRecording, setIsRecording] = useState(false)
  const [isPaused, setIsPaused] = useState(false)
  const [elapsedSeconds, setElapsedSeconds] = useState(0)
  const [showModal, setShowModal] = useState(false)
  const [bars, setBars] = useState<number[]>(() =>
    Array.from({ length: TOTAL_BARS }, () => 4)
  )
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null)
  const animationRef = useRef<ReturnType<typeof setInterval> | null>(null)

  const formatTime = (totalSeconds: number) => {
    const hrs = String(Math.floor(totalSeconds / 3600)).padStart(2, "0")
    const mins = String(Math.floor((totalSeconds % 3600) / 60)).padStart(2, "0")
    const secs = String(totalSeconds % 60).padStart(2, "0")
    return `${hrs}:${mins}:${secs}`
  }

  const startTimerAndAnimation = useCallback(() => {
    intervalRef.current = setInterval(() => {
      setElapsedSeconds((s) => s + 1)
    }, 1000)
    animationRef.current = setInterval(() => {
      setBars(
        Array.from({ length: TOTAL_BARS }, () =>
          Math.floor(Math.random() * 28) + 4
        )
      )
    }, 150)
  }, [])

  const stopTimerAndAnimation = useCallback(() => {
    if (intervalRef.current) clearInterval(intervalRef.current)
    if (animationRef.current) clearInterval(animationRef.current)
    intervalRef.current = null
    animationRef.current = null
  }, [])

  useEffect(() => {
    return () => stopTimerAndAnimation()
  }, [stopTimerAndAnimation])

  const handleStart = () => {
    setIsRecording(true)
    setIsPaused(false)
    setElapsedSeconds(0)
    startTimerAndAnimation()
  }

  const handlePause = () => {
    setIsPaused(true)
    stopTimerAndAnimation()
    setBars(Array.from({ length: TOTAL_BARS }, () => 4))
  }

  const handleResume = () => {
    setIsPaused(false)
    startTimerAndAnimation()
  }

  const handleStopClick = () => {
    setShowModal(true)
  }

  const handleConfirmStop = () => {
    setIsRecording(false)
    setIsPaused(false)
    stopTimerAndAnimation()
    setElapsedSeconds(0)
    setBars(Array.from({ length: TOTAL_BARS }, () => 4))
    setShowModal(false)
    // Aquí iría la lógica para enviar el audio a analizar
  }

  const handleCancelStop = () => {
    setShowModal(false)
  }

  if (!isRecording) {
    return (
      <div className="record">
        <button className="record__start-btn" onClick={handleStart}>
          <span className="record__start-btn-icon">&#9654;&#10073;&#10073;</span>
          Iniciar grabación
        </button>
      </div>
    )
  }

  return (
    <div className="record">
      <h1 className="h2 record__title">Grabación en curso</h1>
      <p className="paragraph record__subtitle">
        Puedes minimizar la app, continuaremos grabando. Si cierras la app se
        detendrá la grabación automáticamente.
      </p>

      <div className="record__waveform">
        {bars.map((height, i) => (
          <div
            key={i}
            className="record__waveform-bar"
            style={{ height: `${height}px` }}
          />
        ))}
      </div>

      <span className="h2 record__timer">{formatTime(elapsedSeconds)}</span>

      <div className="record__actions">
        <button
          className="record__btn record__btn--outline"
          onClick={isPaused ? handleResume : handlePause}
        >
          <span className="record__btn-icon">
            {isPaused ? "\u25B6" : "\u275A\u275A"}
          </span>
          {isPaused ? "Reanudar grabación" : "Pausar grabación"}
        </button>
        <button
          className="record__btn record__btn--filled"
          onClick={handleStopClick}
        >
          <span className="record__btn-icon">&#9632;</span>
          Finalizar grabación
        </button>
      </div>

      <Modal
        isOpen={showModal}
        onClose={handleCancelStop}
        title="¿Quieres finalizar la grabación?"
        description="Al aceptar se enviará a analizar y te devolveremos información al respecto."
        confirmText="Finalizar grabación"
        onConfirm={handleConfirmStop}
      />
    </div>
  )
}

export default Record
