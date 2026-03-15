import type { WorkerProgress } from './shared_types'
import { ensureInit } from '#image/wasm/init'
import { parseWorkerError, WorkerMessageType } from './shared_types'

type ProgressCallback = (update: WorkerProgress) => void

export function createWorker<TRequest, TResult>(
  handler: (request: TRequest, callback: ProgressCallback) => TResult,
): void {
  const callback: ProgressCallback = (update) => {
    globalThis.postMessage({ type: WorkerMessageType.PROGRESS, payload: update })
  }

  globalThis.addEventListener('message', (e: MessageEvent<TRequest>) => {
    callback({ progress: 0, message: 'Initializing...' })
    ensureInit().then(() => {
      try {
        const result = handler(e.data, callback)
        globalThis.postMessage({ type: WorkerMessageType.DONE, payload: { success: true, data: result } })
      }
      catch (err) {
        globalThis.postMessage({ type: WorkerMessageType.ERROR, payload: { success: false, error: parseWorkerError(err) } })
      }
    }).catch((err) => {
      globalThis.postMessage({ type: WorkerMessageType.ERROR, payload: { success: false, error: parseWorkerError(err) } })
    })
  })
}
