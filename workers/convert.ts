import init, { convertImage } from '#image/wasm/pkg/wasm_convert'
import { type WorkerMessage, WorkerMessageType, type WorkerRequest, type WorkerResponse } from './convert.d'

globalThis.addEventListener(
  'message',
  (e: MessageEvent<WorkerRequest>) => {
    init().then(() => {
      // eslint-disable-next-line ts/no-unsafe-assignment
      const { inputFile, inputType, outputType, settings } = e.data

      const res = convertImage(inputFile, inputType, outputType, callback, settings)

      const response: WorkerResponse = {
        success: true,
        data: res,
      }

      globalThis.postMessage({
        type: WorkerMessageType.DONE,
        payload: response,
      } as WorkerMessage)
    }).catch((e) => {
      const response: WorkerResponse = {
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
  } as WorkerMessage)
}
