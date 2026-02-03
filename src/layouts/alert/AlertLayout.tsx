import "./alertLayout.css"
import { useAlertStore } from "../../stores/alert.store"
import Alert from "../../components/ui/alert/Alert"

const AlertLayout = () => {
  const { alerts, removeAlert } = useAlertStore()

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
            onActionBtn1={() => removeAlert(alert.id)}
          />))
        }
      </div>
    </div>
  )
}

export default AlertLayout