import type { SaveMetadataWorkerRequest } from './save_metadata.d'
import { saveMetadata } from '#image/wasm/pkg/web/refilelabs_image'
import { createWorker } from './create_worker'

createWorker<SaveMetadataWorkerRequest, Uint8Array>(({ inputFile, inputType, changes, stripAll, stripGps }, cb) =>
  saveMetadata(inputFile, inputType, changes, stripAll, stripGps, cb))
