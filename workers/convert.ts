import type { ConvertWorkerMessage, ConvertWorkerRequest, ConvertWorkerResponse } from './convert.d'
import init, { convertImage } from '#image/wasm/pkg/image'

import { WorkerMessageType } from './shared_types'

globalThis.addEventListener(
  'message',
  (e: MessageEvent<ConvertWorkerRequest>) => {
    init().then(() => {
      // eslint-disable-next-line ts/no-unsafe-assignment
      const { inputFile, inputType, outputType, settings } = e.data

      const res = convertImage(inputFile, inputType, outputType, callback, settings)

      const response: ConvertWorkerResponse = {
        success: true,
        data: res,
      }

      globalThis.postMessage({
        type: WorkerMessageType.DONE,
        payload: response,
      } as ConvertWorkerMessage)
    }).catch((e) => {
      const response: ConvertWorkerResponse = {
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
  } as ConvertWorkerMessage)
}
