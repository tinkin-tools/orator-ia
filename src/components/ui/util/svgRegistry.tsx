import React from "react"
import CheckCircle from "../../../assets/icons/check_circle.svg?react"
import XMark from "../../../assets/icons/x_mark.svg?react"
import XCircle from "../../../assets/icons/x_circle.svg?react"
import ExclamationTriangle from "../../../assets/icons/exclamation_triangle.svg?react"
import InformationCircle from "../../../assets/icons/information_circle.svg?react"
import { IconName } from "../icon/iconName"

type SvgComponent = React.FC<React.SVGProps<SVGSVGElement>>

export const svgRegistry: Record<IconName, SvgComponent> = {
  [IconName.CheckCircle]: CheckCircle,
  [IconName.XMark]: XMark,
  [IconName.XCircle]: XCircle,
  [IconName.ExclamationTriangle]: ExclamationTriangle,
  [IconName.InformationCircle]: InformationCircle,
}
