export type CommandPolicy = {
  allowed: string[]
}

export const DEFAULT_ALLOWED_COMMANDS = [
  "cargo fmt",
  "cargo check",
  "cargo clippy",
  "cargo test",
  "cargo metadata",
  "rg",
  "git diff",
]

function matchesAllowed(command: string, allowed: string[]) {
  return allowed.some((pattern) => {
    const normalized = pattern.trim()
    if (!normalized) return false
    const withWildcard = normalized.includes("*") ? normalized : `${normalized}*`
    return match(command, withWildcard)
  })
}

function match(input: string, pattern: string) {
  let escaped = pattern
    .replace(/[.+^${}()|[\]\\]/g, "\\$&")
    .replace(/\*/g, ".*")
    .replace(/\?/g, ".")

  if (escaped.endsWith(" .*")) {
    escaped = escaped.slice(0, -3) + "( .*)?"
  }

  return new RegExp("^" + escaped + "$", "s").test(input)
}

export function assertAllowed(command: string[], policy: CommandPolicy) {
  const joined = command.join(" ")
  if (!matchesAllowed(joined, policy.allowed)) {
    throw new Error(`Command not allowed by policy: ${joined}`)
  }
}

export function bashPermissionPatterns(policy: CommandPolicy) {
  return policy.allowed.map((pattern) => (pattern.includes("*") ? pattern : `${pattern}*`))
}
