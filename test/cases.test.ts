import path from "path"
import fs from "fs/promises"
import { describe, expect, test } from "bun:test"
import { repairRustProject } from "../src/repair"

async function mkTempDir(prefix: string) {
  const base = path.join(process.cwd(), ".fixer", "tmp")
  await fs.mkdir(base, { recursive: true })
  return fs.mkdtemp(path.join(base, prefix))
}

describe("tier1 cases", () => {
  test("basic crate passes cargo check", async () => {
    const caseDir = path.join(process.cwd(), "cases", "tier1", "basic")
    const outputDir = await mkTempDir("tier1-basic-")
    const result = await repairRustProject({
      workspaceDir: caseDir,
      outputDir,
      constraints: {
        maxIterations: 0,
        requireCargoTest: false,
      },
    })
    expect(result.status).toBe("success")
    expect(result.metrics.cargoCheckPass).toBe(true)
  })
})

async function listCaseDirs(tier: string) {
  const tierDir = path.join(process.cwd(), "cases", tier)
  const entries = await fs.readdir(tierDir, { withFileTypes: true }).catch(() => [])
  return entries.filter((e) => e.isDirectory()).map((e) => path.join(tierDir, e.name))
}

const tier2Enabled = process.env.FIXER_TIER === "2"
const tier3Enabled = process.env.FIXER_TIER === "3"

const tier2Test = tier2Enabled ? test : test.skip
const tier3Test = tier3Enabled ? test : test.skip

describe("tier2 cases", () => {
  tier2Test("run tier2 cases when enabled", async () => {
    const cases = await listCaseDirs("tier2")
    for (const caseDir of cases) {
      const outputDir = await mkTempDir("tier2-")
      const result = await repairRustProject({
        workspaceDir: caseDir,
        outputDir,
        constraints: {
          maxIterations: 0,
          requireCargoTest: false,
        },
      })
      expect(result.metrics.cargoCheckPass).toBe(true)
    }
  })
})

describe("tier3 cases", () => {
  tier3Test("run tier3 cases when enabled", async () => {
    const cases = await listCaseDirs("tier3")
    for (const caseDir of cases) {
      const outputDir = await mkTempDir("tier3-")
      const result = await repairRustProject({
        workspaceDir: caseDir,
        outputDir,
        constraints: {
          maxIterations: 0,
          requireCargoTest: false,
        },
      })
      expect(result.metrics.cargoCheckPass).toBe(true)
    }
  })
})
