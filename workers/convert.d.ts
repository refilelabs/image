import type { WorkerMessage, WorkerRequest, WorkerResponse } from './shared_types'
import type { Settings } from '~/utils/settings'

export type ConvertWorkerResponse = WorkerResponse<Uint8Array>
export type ConvertWorkerMessage = WorkerMessage<ConvertWorkerResponse>

export interface ConvertWorkerRequest extends WorkerRequest {
  outputType: string
  settings?: Settings
}
