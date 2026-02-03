import React from "react"
import ReactDOM from "react-dom/client"
import App from "./App"
import ToastLayout from "./layouts/toast/ToastLayout"
import AlertLayout from "./layouts/alert/AlertLayout"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ToastLayout />
    <AlertLayout />
    <App />
  </React.StrictMode>
)
