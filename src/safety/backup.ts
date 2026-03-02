/**
 * Backup System - Create and restore backups to prevent file corruption
 */

import fs from "fs/promises"
import path from "path"

export interface BackupPoint {
  id: string
  timestamp: number
  iteration: number
  path: string
  errorCount: number
}

export class BackupManager {
  private baseDir: string
  private backups: BackupPoint[] = []
  private maxBackups: number

  constructor(outputDir: string, maxBackups = 5) {
    this.baseDir = path.join(outputDir, ".fixer", "backups")
    this.maxBackups = maxBackups
  }

  /**
   * Initialize backup directory
   */
  async init(): Promise<void> {
    await fs.mkdir(this.baseDir, { recursive: true })
  }

  /**
   * Create a backup of the current state
   */
  async createBackup(
    outputDir: string,
    iteration: number,
    errorCount: number
  ): Promise<BackupPoint> {
    const id = `backup-${iteration}-${Date.now()}`
    const backupPath = path.join(this.baseDir, id)

    // Copy src directory
    const srcDir = path.join(outputDir, "src")
    const backupSrcDir = path.join(backupPath, "src")

    await this.copyDir(srcDir, backupSrcDir)

    const backup: BackupPoint = {
      id,
      timestamp: Date.now(),
      iteration,
      path: backupPath,
      errorCount,
    }

    this.backups.push(backup)

    // Clean up old backups
    await this.cleanupOldBackups()

    return backup
  }

  /**
   * Restore from a backup point
   */
  async restore(backup: BackupPoint, outputDir: string): Promise<void> {
    const srcDir = path.join(outputDir, "src")
    const backupSrcDir = path.join(backup.path, "src")

    // Remove current src
    await fs.rm(srcDir, { recursive: true, force: true })

    // Copy backup src
    await this.copyDir(backupSrcDir, srcDir)
  }

  /**
   * Get the most recent backup
   */
  getLatestBackup(): BackupPoint | null {
    if (this.backups.length === 0) return null
    return this.backups[this.backups.length - 1]
  }

  /**
   * Get backup by iteration
   */
  getBackupByIteration(iteration: number): BackupPoint | null {
    return this.backups.find(b => b.iteration === iteration) ?? null
  }

  /**
   * Find the best backup to restore (lowest error count)
   */
  getBestBackup(): BackupPoint | null {
    if (this.backups.length === 0) return null
    return this.backups.reduce((best, current) =>
      current.errorCount < best.errorCount ? current : best
    )
  }

  /**
   * Clean up old backups to save disk space
   */
  private async cleanupOldBackups(): Promise<void> {
    while (this.backups.length > this.maxBackups) {
      const oldest = this.backups.shift()
      if (oldest) {
        try {
          await fs.rm(oldest.path, { recursive: true, force: true })
        } catch {
          // Ignore cleanup errors
        }
      }
    }
  }

  /**
   * Copy a directory recursively
   */
  private async copyDir(src: string, dest: string): Promise<void> {
    await fs.mkdir(dest, { recursive: true })

    const entries = await fs.readdir(src, { withFileTypes: true })

    for (const entry of entries) {
      const srcPath = path.join(src, entry.name)
      const destPath = path.join(dest, entry.name)

      if (entry.isDirectory()) {
        // Skip certain directories
        if (entry.name === "target" || entry.name === ".git") continue
        await this.copyDir(srcPath, destPath)
      } else if (entry.isFile()) {
        await fs.copyFile(srcPath, destPath)
      }
    }
  }

  /**
   * List all backups
   */
  listBackups(): BackupPoint[] {
    return [...this.backups]
  }

  /**
   * Clear all backups
   */
  async clearAll(): Promise<void> {
    for (const backup of this.backups) {
      try {
        await fs.rm(backup.path, { recursive: true, force: true })
      } catch {
        // Ignore
      }
    }
    this.backups = []
  }
}

// Global backup manager instance
let globalBackupManager: BackupManager | null = null

export function getBackupManager(outputDir: string): BackupManager {
  if (!globalBackupManager) {
    globalBackupManager = new BackupManager(outputDir)
  }
  return globalBackupManager
}

export async function resetBackupManager(): Promise<void> {
  if (globalBackupManager) {
    await globalBackupManager.clearAll()
    globalBackupManager = null
  }
}
