import type { WorkerRequest } from './shared_types'
import { getPixels } from '#image/wasm/pkg/web/refilelabs_image'
import { createWorker } from './create_worker'

createWorker<WorkerRequest, ReturnType<typeof getPixels>>(({ inputFile, inputType }) =>
  getPixels(inputFile, inputType))
