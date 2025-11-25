import "./tab.css"
import React from "react"

export interface TabProps {
  label: string
  isActive: boolean
  onClick: () => void
}

const Tab: React.FC<TabProps> = ({ label, onClick, isActive = false }) => {
  const getClassName = (): string =>
    `tab tab-${isActive ? "active" : "inactive"} small-semibold`
  return (
    <button className={getClassName()} onClick={onClick}>
      {label}
    </button>
  )
}

export default Tab
