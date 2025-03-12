import type { MetadataWorkerMessage, MetadataWorkerRequest, MetadataWorkerResponse } from './metadata.d'
import init, { loadMetadata } from '#image/wasm/pkg/refilelabs_image'

import { WorkerMessageType } from './shared_types'

globalThis.addEventListener(
  'message',
  (e: MessageEvent<MetadataWorkerRequest>) => {
    callback(0, 'Initializing...')

    init().then(() => {
      const { inputFile, inputType } = e.data

      const res = loadMetadata(inputFile, inputType, callback)

      const response: MetadataWorkerResponse = {
        success: true,
        data: res,
      }

      globalThis.postMessage({
        type: WorkerMessageType.DONE,
        payload: response,
      } as MetadataWorkerMessage)
    }).catch((e) => {
      const response: MetadataWorkerResponse = {
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
  } as MetadataWorkerMessage)
}
