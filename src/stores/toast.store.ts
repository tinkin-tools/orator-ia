import { create } from "zustand"
import { ToastVariant } from "../types/toast.types"

export interface ToastItem {
  id: string
  description: string
  variant: ToastVariant
  onClose: () => void
}

interface ToastStore {
  toasts: ToastItem[]
  addToast: (toast: ToastItem) => void
  removeToast: (id: string) => void
}

export const useToastStore = create<ToastStore>()((set) => ({
  toasts: [],
  addToast: (toast) => set((state) => ({ toasts: [...state.toasts, toast] })),
  removeToast: (id) =>
    set((state) => ({
      toasts: state.toasts.filter((value) => value.id != id),
    })),
}))
