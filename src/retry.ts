/**
 * Retry Module
 *
 * Handles transient errors with intelligent retry logic.
 * Based on OpenCode's SessionRetry mechanism.
 */

export const RETRYABLE_ERRORS = [
  "rate_limit",
  "timeout",
  "connection_error",
  "overloaded",
  "负载较高",
  "load",
  "AI_RetryError",
  "No response",
  "模型无返回结果",
  "ECONNRESET",
  "ETIMEDOUT",
  "503",
  "502",
  "500",
]

export const MAX_RETRIES = 3

/**
 * Check if an error is retryable
 */
export function isRetryableError(error: unknown): boolean {
  const message =
    error instanceof Error
      ? error.message
      : typeof error === "string"
        ? error
        : JSON.stringify(error)

  return RETRYABLE_ERRORS.some(e => message.toLowerCase().includes(e.toLowerCase()))
}

/**
 * Get delay before retry using exponential backoff
 * @param attempt Current attempt number (1-based)
 * @returns Delay in milliseconds
 */
export function getRetryDelay(attempt: number): number {
  // Exponential backoff: 1s, 2s, 4s, 8s, max 30s
  return Math.min(1000 * Math.pow(2, attempt - 1), 30000)
}

/**
 * Sleep for a given number of milliseconds
 */
export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms))
}

/**
 * Execute a function with automatic retry on transient errors
 */
export async function withRetry<T>(
  fn: () => Promise<T>,
  options?: {
    maxRetries?: number
    onRetry?: (error: unknown, attempt: number, delay: number) => void
  }
): Promise<T> {
  const maxRetries = options?.maxRetries ?? MAX_RETRIES
  let lastError: unknown

  for (let attempt = 1; attempt <= maxRetries + 1; attempt++) {
    try {
      return await fn()
    } catch (error) {
      lastError = error

      if (attempt > maxRetries || !isRetryableError(error)) {
        throw error
      }

      const delay = getRetryDelay(attempt)
      options?.onRetry?.(error, attempt, delay)
      await sleep(delay)
    }
  }

  throw lastError
}
