import type { SaveMetadataWorkerRequest } from './save_metadata.d'
import { saveMetadata } from '#image/wasm/pkg/refilelabs_image'
import { createWorker } from './create_worker'

createWorker<SaveMetadataWorkerRequest, Uint8Array>(({ inputFile, inputType, changes, stripGps }, cb) =>
  saveMetadata(inputFile, inputType, changes, stripGps ?? false, cb))
