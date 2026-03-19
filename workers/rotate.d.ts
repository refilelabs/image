import type { WorkerMessage, WorkerRequest, WorkerResponse } from './shared_types'

export type RotateWorkerResponse = WorkerResponse<Uint8Array>
export type RotateWorkerMessage = WorkerMessage<RotateWorkerResponse>

export interface RotateWorkerRequest extends WorkerRequest {
  degrees: number
  flipH: boolean
  flipV: boolean
}
