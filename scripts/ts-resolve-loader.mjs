import path from "path"
import fs from "fs/promises"
import { fileURLToPath, pathToFileURL } from "url"

export async function resolve(specifier, context, nextResolve) {
  if (specifier.startsWith(".") && !path.extname(specifier)) {
    try {
      return await nextResolve(`${specifier}.ts`, context)
    } catch {}
    try {
      return await nextResolve(`${specifier}.js`, context)
    } catch {}
    try {
      const baseUrl = new URL(specifier, context.parentURL)
      const basePath = fileURLToPath(baseUrl)
      const stat = await fs.stat(basePath).catch(() => undefined)
      if (stat?.isDirectory()) {
        try {
          return await nextResolve(pathToFileURL(path.join(basePath, "index.ts")).href, context)
        } catch {}
        return await nextResolve(pathToFileURL(path.join(basePath, "index.js")).href, context)
      }
    } catch {}
  }
  return nextResolve(specifier, context)
}
