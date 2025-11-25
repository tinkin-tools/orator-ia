import "./styles/index.css"
import { useState } from "react"
import NavBar from "./components/ui/navBar/NavBar"
import Dashboard from "./pages/dashboard/Dashboard"
import Record from "./pages/record/Record"
import History from "./pages/history/History"

type Tab = "dashboard" | "record" | "history"

function App() {
  const [activeTab, setActiveTab] = useState<Tab>("dashboard")

  const buildContent = () => {
    switch (activeTab) {
      case "dashboard":
        return <Dashboard />
      case "record":
        return <Record />
      case "history":
        return <History />
    }
  }

  return (
    <>
      <NavBar
        tabs={[
          {
            label: "Dashboard",
            isActive: activeTab == "dashboard",
            onClick: () => setActiveTab("dashboard"),
          },
          {
            label: "Nueva Grabación",
            isActive: activeTab == "record",
            onClick: () => setActiveTab("record"),
          },
          {
            label: "Historial",
            isActive: activeTab == "history",
            onClick: () => setActiveTab("history"),
          },
        ]}
      />
      {buildContent()}
    </>
  )
}

export default App
