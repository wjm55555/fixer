/**
 * Box Drawing - Create beautiful terminal boxes and borders
 */

import { Colors } from "./colors.ts"

export type BoxStyle = "single" | "double" | "rounded"

const BOX_CHARS = {
  single: {
    topLeft: "┌",
    topRight: "┐",
    bottomLeft: "└",
    bottomRight: "┘",
    horizontal: "─",
    vertical: "│",
    cross: "┼",
    teeLeft: "├",
    teeRight: "┤",
    teeTop: "┬",
    teeBottom: "┴",
  },
  double: {
    topLeft: "╔",
    topRight: "╗",
    bottomLeft: "╚",
    bottomRight: "╝",
    horizontal: "═",
    vertical: "║",
    cross: "╬",
    teeLeft: "╠",
    teeRight: "╣",
    teeTop: "╦",
    teeBottom: "╩",
  },
  rounded: {
    topLeft: "╭",
    topRight: "╮",
    bottomLeft: "╰",
    bottomRight: "╯",
    horizontal: "─",
    vertical: "│",
    cross: "┼",
    teeLeft: "├",
    teeRight: "┤",
    teeTop: "┬",
    teeBottom: "┴",
  },
}

export interface BoxOptions {
  width?: number
  style?: BoxStyle
  padding?: number
  title?: string
  color?: string
}

/**
 * Draw a box around text content
 */
export function drawBox(content: string[], options: BoxOptions = {}): string {
  const {
    width = 65,
    style = "rounded",
    padding = 1,
    title,
    color = Colors.cyan,
  } = options

  const chars = BOX_CHARS[style]
  const innerWidth = width - 2 // Account for borders

  // Pad each line
  const padLine = (text: string) => {
    const stripped = stripAnsi(text)
    const ansiLength = text.length - stripped.length
    const padded = text.padEnd(innerWidth + ansiLength + padding * 2)
    return `${chars.vertical}${" ".repeat(padding)}${padded}${" ".repeat(padding)}${chars.vertical}`
  }

  // Top border
  const lines: string[] = []
  if (title) {
    const titleText = ` ${title} `
    const titleLen = stripAnsi(titleText).length
    const leftPad = Math.floor((innerWidth - titleLen) / 2)
    const rightPad = innerWidth - titleLen - leftPad
    lines.push(
      `${color}${chars.topLeft}${chars.horizontal.repeat(leftPad)}${titleText}${chars.horizontal.repeat(rightPad)}${chars.topRight}${Colors.reset}`
    )
  } else {
    lines.push(
      `${color}${chars.topLeft}${chars.horizontal.repeat(innerWidth)}${chars.topRight}${Colors.reset}`
    )
  }

  // Content
  for (const line of content) {
    lines.push(`${color}${padLine(line)}${Colors.reset}`)
  }

  // Bottom border
  lines.push(
    `${color}${chars.bottomLeft}${chars.horizontal.repeat(innerWidth)}${chars.bottomRight}${Colors.reset}`
  )

  return lines.join("\n")
}

/**
 * Draw a simple divider line
 */
export function drawDivider(width = 65, style: BoxStyle = "single", color = Colors.gray): string {
  const chars = BOX_CHARS[style]
  return `${color}${chars.horizontal.repeat(width)}${Colors.reset}`
}

/**
 * Draw a header with divider
 */
export function drawHeader(text: string, width = 65, style: BoxStyle = "single"): string {
  const chars = BOX_CHARS[style]
  const textLen = stripAnsi(text).length
  const leftPad = Math.floor((width - textLen - 2) / 2)
  const rightPad = width - textLen - 2 - leftPad

  return [
    `${chars.topLeft}${chars.horizontal.repeat(width - 2)}${chars.topRight}`,
    `${chars.vertical}${" ".repeat(leftPad)}${text}${" ".repeat(rightPad)}${chars.vertical}`,
    `${chars.teeLeft}${chars.horizontal.repeat(width - 2)}${chars.teeRight}`,
  ].join("\n")
}

/**
 * Strip ANSI escape codes for accurate length calculation
 */
function stripAnsi(text: string): string {
  return text.replace(/\x1b\[[0-9;]*m/g, "")
}

/**
 * Create a tree structure display
 */
export function tree(items: Array<{ text: string; children?: Array<{ text: string }> }>): string {
  const lines: string[] = []

  items.forEach((item, index) => {
    const isLast = index === items.length - 1
    const prefix = isLast ? "└─" : "├─"

    lines.push(`${prefix} ${item.text}`)

    if (item.children) {
      item.children.forEach((child, childIndex) => {
        const isLastChild = childIndex === item.children!.length - 1
        const childPrefix = isLast ? "  " : "│ "
        const childBranch = isLastChild ? "└─" : "├─"
        lines.push(`${childPrefix}${childBranch} ${child.text}`)
      })
    }
  })

  return lines.join("\n")
}
