import "./alertLayout.css"
import { useAlertStore } from "../../stores/alert.store"
import Alert from "../../components/ui/alert/Alert"

const AlertLayout = () => {
  const { alerts } = useAlertStore()

  return (
    <div className="alert-layout">
      <div className="alert-container">
        {alerts.map((alert) => (
          <Alert
            key={alert.id}
            id={alert.id}
            title={alert.title}
            description={alert.description}
            variant={alert.variant}
            actionBtn1Label={alert.actionBtn1Label}
            actionBtn2Label={alert.actionBtn2Label}
            onActionBtn1={alert.onActionBtn1}
            onActionBtn2={alert.onActionBtn2}
            onDetails={alert.onDetails}
          />
        ))}
      </div>
    </div>
  )
}

export default AlertLayout