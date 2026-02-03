import { create } from "zustand"
import { AlertVariant } from "../types/alert.types"

export interface AlertItem {
  id: string
  title: string
  description: string
  variant: AlertVariant
  onActionBtn1: () => void
  onActionBtn2?: () => void
  onDetails?: () => void
}

interface AlertStore {
  alerts: AlertItem[]
  addAlert: (alert: AlertItem) => void
  removeAlert: (id: string) => void
}

export const useAlertStore = create<AlertStore>()((set) => ({
  alerts: [],
  addAlert: (alert) => set((state) => ({ alerts: [...state.alerts, alert] })),
  removeAlert: (id) =>
    set((state) => ({
      alerts: state.alerts.filter((value) => value.id != id),
    })),
}))