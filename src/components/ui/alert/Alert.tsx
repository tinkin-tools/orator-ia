import { AlertVariant } from "../../../types/alert.types"
import Icon from "../icon/Icon"
import { IconName } from "../icon/iconName"
import "./alert.css"

interface AlertProps {
  id: string
  title: string
  description: string
  variant: AlertVariant
  actionBtn1Label: string
  actionBtn2Label: string
  onActionBtn1: () => void
  onActionBtn2?: () => void
  onDetails?: () => void
}

const Alert: React.FC<AlertProps> = ({
  title,
  description,
  variant,
  actionBtn1Label,
  actionBtn2Label,
  onActionBtn1,
  onActionBtn2,
  onDetails,
}) => {
  const getVariantClass = () => {
    return `alert alert--${variant}`
  }

  const getIcon = () => {
    switch (variant) {
      case AlertVariant.Warning:
        return <Icon name={IconName.ExclamationTriangle} />
      case AlertVariant.Error:
        return <Icon name={IconName.XCircle} />
      case AlertVariant.Success:
        return <Icon name={IconName.CheckCircle} />
      case AlertVariant.Info:
        return <Icon name={IconName.InformationCircle} />
      default:
        return null
    }
  }

  return (
    <div className={getVariantClass()}>
      <div className={`alert__icon alert__icon--${variant}`}>
        {getIcon()}
      </div>
      <div className="alert_text__content">
        <h4 className="alert__title">{title}</h4>
        <p className="alert__description">{description}</p>
        <div className="alert__btn_container">
          <p
            className="alert__btn"
            onClick={onActionBtn1}
          >
            {actionBtn1Label}
          </p>
          <p
            className="alert__btn"
            onClick={onActionBtn2}
          >
            {actionBtn2Label}
          </p>
        </div>
      </div>
      <div>
        <p className="details__btn" onClick={onDetails}>Details </p>
      </div>
    </div>
  )
}

export default Alert
