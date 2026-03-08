export enum WorkerMessageType {
  PROGRESS,
  DONE,
  ERROR,
}

export type WorkerResponse<T> = {
  success: true
  data: T
} | {
  success: false
  error: string
}

export interface WorkerProgress {
  progress: number
  message: string
}

export type WorkerMessage<T extends WorkerResponse<any>> = {
  type: WorkerMessageType.DONE
  payload: T & { success: true }
} | {
  type: WorkerMessageType.ERROR
  payload: T & { success: false }
} | {
  type: WorkerMessageType.PROGRESS
  payload: WorkerProgress
}

export interface WorkerRequest {
  inputFile: Uint8Array
  inputType: string
}

const errRegex = /^Error: /

export function parseWorkerError(e: unknown): string {
  let msg: string
  if (e instanceof Error) {
    msg = e.message
  }
  else {
    msg = String(e)
  }
  return msg.replace(errRegex, '')
}
