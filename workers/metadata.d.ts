import type { Metadata } from '#image/wasm/pkg/bundler/refilelabs_image'
import type { WorkerMessage, WorkerRequest, WorkerResponse } from './shared_types'

export type MetadataWorkerResponse = WorkerResponse<Metadata>
export type MetadataWorkerMessage = WorkerMessage<MetadataWorkerResponse>

export type MetadataWorkerRequest = WorkerRequest
