import type { CropWorkerRequest } from './crop.d'
import { cropImage } from '#image/wasm/pkg/web/refilelabs_image'
import { createWorker } from './create_worker'

createWorker<CropWorkerRequest, Uint8Array>(({ inputFile, inputType, x, y, width, height }, cb) =>
  cropImage(inputFile, inputType, x, y, width, height, cb))
