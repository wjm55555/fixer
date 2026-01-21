export type RepairConstraints = {
  maxIterations?: number
  timeBudgetMs?: number
  requireCargoTest?: boolean
  allowedCommands?: string[]
  maxLogBytes?: number
  maxWorkspaceBytes?: number
}

export type RepairInput = {
  workspaceDir: string
  outputDir: string
  sourceDir?: string
  constraints?: RepairConstraints
}

export type RepairMetrics = {
  iterations: number
  cargoCheckPass: boolean
  cargoTestPass: boolean
  clippyFixApplied: boolean
}

export type RepairArtifacts = {
  outputDir: string
  logsDir?: string
  patchesDir?: string
  patches?: string[]
}

export type RepairResult = {
  status: "success" | "partial" | "failed"
  summary?: string
  diff?: string
  changedFiles: string[]
  metrics: RepairMetrics
  artifacts: RepairArtifacts
}
