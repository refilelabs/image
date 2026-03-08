import type { ResizeWorkerMessage, ResizeWorkerRequest, ResizeWorkerResponse } from './resize.d'
import init, { resizeImage } from '#image/wasm/pkg/refilelabs_image'

import { WorkerMessageType } from './shared_types'

globalThis.addEventListener(
  'message',
  (e: MessageEvent<ResizeWorkerRequest>) => {
    callback(0, 'Initializing...')

    init().then(() => {
      const { inputFile, inputType, width, height } = e.data

      const res = resizeImage(inputFile, inputType, width, height, callback)

      const response: ResizeWorkerResponse = {
        success: true,
        data: res,
      }

      globalThis.postMessage({
        type: WorkerMessageType.DONE,
        payload: response,
      } as ResizeWorkerMessage)
    }).catch((e) => {
      const response: ResizeWorkerResponse = {
        success: false,
        error: String(e),
      }

      globalThis.postMessage({
        type: WorkerMessageType.ERROR,
        payload: response,
      })
    })
  },
  false,
)

function callback(progress: number, message: string) {
  globalThis.postMessage({
    type: WorkerMessageType.PROGRESS,
    payload: {
      progress,
      message,
    },
  } as ResizeWorkerMessage)
}
