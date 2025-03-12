import type { Settings } from '#image/wasm/pkg/refilelabs_image'
import type { WorkerMessage, WorkerRequest, WorkerResponse } from './shared_types'

export type ConvertWorkerResponse = WorkerResponse<Uint8Array>
export type ConvertWorkerMessage = WorkerMessage<ConvertWorkerResponse>

export interface ConvertWorkerRequest extends WorkerRequest {
  outputType: string
  settings?: Settings
}
