import "./icon.css"
import React from "react"
import { IconName } from "./iconName"
import { svgRegistry } from "../util/svgRegistry"

export enum IconSize {
  Small = "sm",
  Medium = "md",
  Large = "lg",
}

interface IconProps {
  name: IconName
  size?: IconSize
}

const Icon: React.FC<IconProps> = ({ name, size = IconSize.Medium }) => {
  const SvgComponent = svgRegistry[name]

  if (!SvgComponent) {
    return null
  }

  return <SvgComponent className={`icon icon--${size}`} />
}

export default Icon
