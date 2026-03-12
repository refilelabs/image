import type { MetadataChanges } from '#image/wasm/pkg/bundler/refilelabs_image'
import type { WorkerMessage, WorkerRequest, WorkerResponse } from './shared_types'

export type SaveMetadataWorkerResponse = WorkerResponse<Uint8Array>
export type SaveMetadataWorkerMessage = WorkerMessage<SaveMetadataWorkerResponse>

export interface SaveMetadataWorkerRequest extends WorkerRequest {
  changes: MetadataChanges
  stripAll: boolean
  stripGps: boolean
}
