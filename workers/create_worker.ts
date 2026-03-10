import type { WorkerProgress } from './shared_types'
import init from '#image/wasm/pkg/refilelabs_image'
import { WorkerMessageType } from './shared_types'

type ProgressCallback = (update: WorkerProgress) => void

export function createWorker<TRequest, TResult>(
  handler: (request: TRequest, callback: ProgressCallback) => TResult,
): void {
  const callback: ProgressCallback = (update) => {
    globalThis.postMessage({ type: WorkerMessageType.PROGRESS, payload: update })
  }

  globalThis.addEventListener('message', (e: MessageEvent<TRequest>) => {
    callback({ progress: 0, message: 'Initializing...' })
    init().then(() => {
      const result = handler(e.data, callback)
      globalThis.postMessage({ type: WorkerMessageType.DONE, payload: { success: true, data: result } })
    }).catch((err) => {
      globalThis.postMessage({ type: WorkerMessageType.ERROR, payload: { success: false, error: String(err) } })
    })
  }, false)
}
