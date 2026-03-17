import "./history.css"
import { useToastStore } from "../../stores/toast.store"
import { ToastVariant } from "../../types/toast.types"
import React from "react"
import { AlertVariant } from "../../types/alert.types"
import { useAlertStore } from "../../stores/alert.store"

export interface HistoryProps { }

const History: React.FC<HistoryProps> = ({ }) => {
  const { addToast, removeToast } = useToastStore()
  const { addAlert, removeAlert } = useAlertStore()

  const onClick = (variant: ToastVariant) => {
    const id = Date.now().toString()
    addToast({
      id: id,
      description: "This is not a drill",
      variant: variant,
      onClose: () => removeToast(id),
    })
  }

  const onAlertBtnClick = (variant: AlertVariant) => {
    const id = Date.now().toString()
    addAlert({
      id: id,
      title: "Alert Title",
      description: "This is an alert",
      variant: variant,
      onActionBtn1: () => removeAlert(id),
    })
  }

  return (
    <>
      <div
        className="history__container"
      >
        <div
          className="history__container__buttons"
        >
          <button onClick={() => onClick(ToastVariant.Error)}>
            Add error toast
          </button>
          <button onClick={() => onClick(ToastVariant.Warning)}>
            Add warning toast
          </button>
          <button onClick={() => onClick(ToastVariant.Info)}>
            Add info toast
          </button>
          <button onClick={() => onClick(ToastVariant.Success)}>
            Add success toast
          </button>
        </div>

        <div
          className="history__container__buttons"
        >
          <button onClick={() => onAlertBtnClick(AlertVariant.Error)}>
            Add error alert
          </button>
          <button onClick={() => onAlertBtnClick(AlertVariant.Warning)}>
            Add warning alert
          </button>
          <button onClick={() => onAlertBtnClick(AlertVariant.Info)}>
            Add info alert
          </button>
          <button onClick={() => onAlertBtnClick(AlertVariant.Success)}>
            Add success alert
          </button>
        </div>
      </div>
    </>
  )
}

export default History
