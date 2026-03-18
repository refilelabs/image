import type { WorkerMessage, WorkerRequest, WorkerResponse } from './shared_types'

export type CropWorkerResponse = WorkerResponse<Uint8Array>
export type CropWorkerMessage = WorkerMessage<CropWorkerResponse>

export interface CropWorkerRequest extends WorkerRequest {
  x: number
  y: number
  width: number
  height: number
}
