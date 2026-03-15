import type { MetadataWorkerRequest } from './metadata.d'
import { loadMetadata } from '#image/wasm/pkg/web/refilelabs_image'
import { createWorker } from './create_worker'

createWorker<MetadataWorkerRequest, ReturnType<typeof loadMetadata>>(({ inputFile, inputType }, cb) =>
  loadMetadata(inputFile, inputType, cb))
