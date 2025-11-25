import "./toast.css"
import React from "react"
import { ToastVariant } from "../../../types/toast.types"
import Icon from "../icon/Icon"
import { IconName } from "../icon/iconName"

interface ToastProps {
  description: string
  variant: ToastVariant
  onClose: () => void
}

const Toast: React.FC<ToastProps> = ({ description, variant, onClose }) => {
  const getVariantClass = () => {
    return `toast toast--${variant}`
  }

  const getIcon = () => {
    switch (variant) {
      case ToastVariant.Warning:
        return <Icon name={IconName.ExclamationTriangle} />
      case ToastVariant.Error:
        return <Icon name={IconName.XCircle} />
      case ToastVariant.Success:
        return <Icon name={IconName.CheckCircle} />
      case ToastVariant.Info:
        return <Icon name={IconName.InformationCircle} />
      default:
        return null
    }
  }

  return (
    <div className={getVariantClass()}>
      <div className="toast__icon"> {getIcon()} </div>
      <p className="toast__description"> {description} </p>

      <button className="toast__close-container" onClick={onClose}>
        <div className="toast__close-icon">
          <Icon name={IconName.XMark} />
        </div>
      </button>
    </div>
  )
}

export default Toast
