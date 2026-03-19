import type { RotateWorkerRequest } from './rotate.d'
import { rotateImage } from '#image/wasm/pkg/web/refilelabs_image'
import { createWorker } from './create_worker'

createWorker<RotateWorkerRequest, Uint8Array>(({ inputFile, inputType, degrees, flipH, flipV }, cb) =>
  rotateImage(inputFile, inputType, degrees, flipH, flipV, cb))
