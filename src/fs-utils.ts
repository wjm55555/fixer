import fs from "fs/promises"
import path from "path"

const SKIP_DIRS = new Set([".git", "target", ".opencode-core"])

export async function ensureEmptyDir(dir: string) {
  try {
    const entries = await fs.readdir(dir)
    if (entries.length > 0) {
      throw new Error(`Output directory is not empty: ${dir}`)
    }
  } catch (error: any) {
    if (error?.code === "ENOENT") {
      await fs.mkdir(dir, { recursive: true })
      return
    }
    throw error
  }
}

export async function copyWorkspace(src: string, dest: string) {
  await fs.mkdir(dest, { recursive: true, mode: 0o755 })
  const entries = await fs.readdir(src, { withFileTypes: true })
  for (const entry of entries) {
    if (entry.isDirectory() && SKIP_DIRS.has(entry.name)) continue
    const from = path.join(src, entry.name)
    const to = path.join(dest, entry.name)
    if (entry.isDirectory()) {
      await copyWorkspace(from, to)
    } else if (entry.isFile()) {
      await fs.copyFile(from, to)
      await fs.chmod(to, 0o644)
    } else if (entry.isSymbolicLink()) {
      const link = await fs.readlink(from)
      await fs.symlink(link, to)
    }
  }
}

export async function dirSizeBytes(root: string, extraSkip: Set<string> = new Set()) {
  let total = 0
  const entries = await fs.readdir(root, { withFileTypes: true })
  for (const entry of entries) {
    if (entry.isDirectory() && (SKIP_DIRS.has(entry.name) || extraSkip.has(entry.name))) {
      continue
    }
    const full = path.join(root, entry.name)
    if (entry.isDirectory()) {
      total += await dirSizeBytes(full, extraSkip)
    } else if (entry.isFile()) {
      const stat = await fs.stat(full)
      total += stat.size
    }
  }
  return total
}
