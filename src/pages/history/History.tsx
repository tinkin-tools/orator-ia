import "./history.css"
import { useToastStore } from "../../stores/toast.store"
import { ToastVariant } from "../../types/toast.types"
import React from "react"

export interface HistoryProps {}

const History: React.FC<HistoryProps> = ({}) => {
  const { addToast, removeToast } = useToastStore()

  const onClick = (variant: ToastVariant) => {
    const id = Date.now().toString()
    addToast({
      id: id,
      description: "This is not a drill",
      variant: variant,
      onClose: () => removeToast(id),
    })
  }

  return (
    <>
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          width: "25%",
          gap: "8px",
        }}
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
    </>
  )
}

export default History
