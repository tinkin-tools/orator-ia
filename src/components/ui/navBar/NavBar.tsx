import "./navBar.css"
import React from "react"
import Logo from "../logo/Logo"
import { LogoName } from "../logo/logoName"
import Tab, { TabProps } from "./Tab"

interface NavBarProps {
  tabs: TabProps[]
}

const NavBar: React.FC<NavBarProps> = ({ tabs = [] }) => {
  return (
    <div className="navbar">
      <Logo name={LogoName.Small} />
      <div className="tabs-container">
        {tabs.map((tab) => (
          <Tab {...tab} />
        ))}
      </div>
    </div>
  )
}

export default NavBar
