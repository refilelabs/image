import type { WorkerMessage, WorkerRequest, WorkerResponse } from './shared_types'

export type ResizeWorkerResponse = WorkerResponse<Uint8Array>
export type ResizeWorkerMessage = WorkerMessage<ResizeWorkerResponse>

export interface ResizeWorkerRequest extends WorkerRequest {
  width: number
  height: number
}
