import "./modal.css"
import React from "react"

export interface ModalProps {
  isOpen: boolean
  onClose: () => void
  title: string
  description: string
  cancelText?: string
  confirmText: string
  onConfirm: () => void
  variant?: "default" | "danger"
}

const Modal: React.FC<ModalProps> = ({
  isOpen,
  onClose,
  title,
  description,
  cancelText = "Cancelar",
  confirmText,
  onConfirm,
  variant = "default",
}) => {
  if (!isOpen) return null

  const handleBackdropClick = (e: React.MouseEvent<HTMLDivElement>) => {
    if (e.target === e.currentTarget) {
      onClose()
    }
  }

  return (
    <div className="modal" onClick={handleBackdropClick}>
      <div className="modal__content">
        <h2 className="h2 modal__title">{title}</h2>
        <p className="paragraph modal__description">{description}</p>

        <div className="modal__actions">
          <button className="modal__btn modal__btn--cancel" onClick={onClose}>
            {cancelText}
          </button>
          <button
            className={`modal__btn modal__btn--confirm ${
              variant === "danger" ? "modal__btn--danger" : ""
            }`}
            onClick={onConfirm}
          >
            {confirmText}
          </button>
        </div>
      </div>
    </div>
  )
}

export default Modal
