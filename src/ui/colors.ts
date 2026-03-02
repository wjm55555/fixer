/**
 * Terminal Colors - ANSI escape codes for beautiful CLI output
 */

export const Colors = {
  // Reset
  reset: "\x1b[0m",

  // Styles
  bold: "\x1b[1m",
  dim: "\x1b[2m",
  italic: "\x1b[3m",
  underline: "\x1b[4m",

  // Foreground colors
  black: "\x1b[30m",
  red: "\x1b[31m",
  green: "\x1b[32m",
  yellow: "\x1b[33m",
  blue: "\x1b[34m",
  magenta: "\x1b[35m",
  cyan: "\x1b[36m",
  white: "\x1b[37m",
  gray: "\x1b[90m",

  // Bright foreground colors
  brightRed: "\x1b[91m",
  brightGreen: "\x1b[92m",
  brightYellow: "\x1b[93m",
  brightBlue: "\x1b[94m",
  brightMagenta: "\x1b[95m",
  brightCyan: "\x1b[96m",
  brightWhite: "\x1b[97m",

  // Background colors
  bgBlack: "\x1b[40m",
  bgRed: "\x1b[41m",
  bgGreen: "\x1b[42m",
  bgYellow: "\x1b[43m",
  bgBlue: "\x1b[44m",
  bgMagenta: "\x1b[45m",
  bgCyan: "\x1b[46m",
  bgWhite: "\x1b[47m",
}

// Helper functions for common styling
export function success(text: string): string {
  return `${Colors.green}${text}${Colors.reset}`
}

export function error(text: string): string {
  return `${Colors.red}${text}${Colors.reset}`
}

export function warning(text: string): string {
  return `${Colors.yellow}${text}${Colors.reset}`
}

export function info(text: string): string {
  return `${Colors.cyan}${text}${Colors.reset}`
}

export function dim(text: string): string {
  return `${Colors.dim}${text}${Colors.reset}`
}

export function bold(text: string): string {
  return `${Colors.bold}${text}${Colors.reset}`
}

export function highlight(text: string): string {
  return `${Colors.bold}${Colors.brightWhite}${text}${Colors.reset}`
}

// Emoji helpers with fallback for terminals that don't support emojis
export const Icons = {
  success: "✅",
  error: "❌",
  warning: "⚠️ ",
  info: "ℹ️ ",
  working: "⏳",
  thinking: "🤔",
  reading: "📖",
  writing: "✏️ ",
  fixing: "🔧",
  searching: "🔍",
  folder: "📁",
  file: "📄",
  code: "💻",
  rocket: "🚀",
  checkmark: "✓",
  cross: "✗",
  arrow: "→",
  bullet: "•",
  branch: "├─",
  corner: "└─",
  line: "│ ",
}
