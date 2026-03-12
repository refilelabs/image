import type { ConvertWorkerRequest } from './convert.d'
import { convertImage } from '#image/wasm/pkg/bundler/refilelabs_image'
import { createWorker } from './create_worker'

createWorker<ConvertWorkerRequest, Uint8Array>(({ inputFile, inputType, outputType, settings }, cb) =>
  convertImage(inputFile, inputType, outputType, cb, settings))
