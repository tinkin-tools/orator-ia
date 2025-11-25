import React from "react"
import { LogoName } from "./logoName"

interface LogoProps {
  name?: LogoName
}

const Logo: React.FC<LogoProps> = ({ name = LogoName.Medium }) => {
  const getClassName = (): string => {
    switch (name) {
      case LogoName.Large:
        return "logo-lg"
      case LogoName.Medium:
        return "logo-md"
      case LogoName.Small:
        return "logo-sm"
    }
  }

  const getImgPath = (): string => {
    switch (name) {
      case LogoName.Large:
        return "src/assets/logo/logo_lg.png"
      case LogoName.Medium:
        return "src/assets/logo/logo_md.png"
      case LogoName.Small:
        return "src/assets/logo/logo_sm.png"
    }
  }

  return (
    <>
      <img className={`logo ${getClassName()}`} src={getImgPath()} />
    </>
  )
}

export default Logo
