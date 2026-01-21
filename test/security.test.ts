import { describe, expect, test } from "bun:test"
import { assertAllowed, bashPermissionPatterns } from "../src/security"

describe("security policy", () => {
  test("allows whitelisted commands", () => {
    expect(() => assertAllowed(["cargo", "check"], { allowed: ["cargo check"] })).not.toThrow()
    expect(() => assertAllowed(["cargo", "check", "--all-targets"], { allowed: ["cargo check"] })).not.toThrow()
  })

  test("rejects non-whitelisted commands", () => {
    expect(() => assertAllowed(["rm", "-rf", "/"], { allowed: ["cargo check"] })).toThrow()
  })

  test("expands bash permission patterns", () => {
    const patterns = bashPermissionPatterns({ allowed: ["cargo check", "rg *"] })
    expect(patterns).toContain("cargo check*")
    expect(patterns).toContain("rg *")
  })
})
