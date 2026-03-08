import type { ResizeWorkerRequest } from './resize.d'
import { resizeImage } from '#image/wasm/pkg/refilelabs_image'
import { createWorker } from './create_worker'

createWorker<ResizeWorkerRequest, Uint8Array>(({ inputFile, inputType, width, height }, cb) =>
  resizeImage(inputFile, inputType, width, height, cb))
