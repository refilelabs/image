/**
 * Image action callback
 * This is a wrapper type for image actions that return a file and metrics
 */
export interface ImageActionResult<T> {
  metrics: T
  file: File
}
