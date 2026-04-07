import "./toastLayout.css"
import { useToastStore } from "../../stores/toast.store"
import Toast from "../../components/ui/toast/Toast"

const ToastLayout = () => {
  const { toasts, removeToast } = useToastStore()

  return (
    <div className="toast-layout">
      <div className="toast-container">
        {toasts.map((toast) => (
          <Toast
            key={toast.id}
            description={toast.description}
            variant={toast.variant}
            onClose={() => removeToast(toast.id)}
          />
        ))}
      </div>
    </div>
  )
}

export default ToastLayout
