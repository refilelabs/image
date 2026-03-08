import init from '#image/wasm/pkg/refilelabs_image'
import { WorkerMessageType } from './shared_types'

type ProgressCallback = (progress: number, message: string) => void

export function createWorker<TRequest, TResult>(
  handler: (request: TRequest, callback: ProgressCallback) => TResult,
): void {
  const callback: ProgressCallback = (progress, message) => {
    globalThis.postMessage({ type: WorkerMessageType.PROGRESS, payload: { progress, message } })
  }

  globalThis.addEventListener('message', (e: MessageEvent<TRequest>) => {
    callback(0, 'Initializing...')
    init().then(() => {
      const result = handler(e.data, callback)
      globalThis.postMessage({ type: WorkerMessageType.DONE, payload: { success: true, data: result } })
    }).catch((err) => {
      globalThis.postMessage({ type: WorkerMessageType.ERROR, payload: { success: false, error: String(err) } })
    })
  }, false)
}
