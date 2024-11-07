import type { Metadata } from '#image/wasm/pkg/image'
import type { WorkerMessage, WorkerRequest, WorkerResponse } from './shared_types'

export type MetadataWorkerResponse = WorkerResponse<Metadata>
export type MetadataWorkerMessage = WorkerMessage<MetadataWorkerResponse>

export interface MetadataWorkerRequest extends WorkerRequest {}
