import type { WorkerMessage, WorkerProgress, WorkerResponse } from '#image/workers/shared_types'
import { WorkerMessageType } from '#image/workers/shared_types'

export async function runWorker<TData>(
  WorkerCtor: new () => Worker,
  params: object,
  onProgress?: (p: WorkerProgress) => void,
  timeout = 10_000,
): Promise<TData> {
  return new Promise((resolve, reject) => {
    const worker = new WorkerCtor()
    const timer = setTimeout(() => {
      worker.terminate()
      reject(new Error('Timed out'))
    }, timeout)

    worker.onmessage = (e: MessageEvent<WorkerMessage<WorkerResponse<TData>>>) => {
      const { type, payload } = e.data
      if (type === WorkerMessageType.PROGRESS) {
        onProgress?.(payload)
      }
      else if (type === WorkerMessageType.DONE) {
        clearTimeout(timer)
        worker.terminate()
        resolve((payload as { success: true, data: TData }).data)
      }
      else if (type === WorkerMessageType.ERROR) {
        clearTimeout(timer)
        worker.terminate()
        reject(new Error((payload as { success: false, error: string }).error))
      }
    }

    worker.onerror = (e) => {
      clearTimeout(timer)
      worker.terminate()
      reject(new Error(e.message))
    }

    worker.postMessage(params)
  })
}
