import type { ImageData as WasmImageData } from '#image/wasm/pkg/web/refilelabs_image'
import { getFileMimeType } from '#image/utils/file_types'
import { runWorker } from '#image/utils/run_worker'
import GetPixelsWorker from '@/workers/get_pixels.ts?worker'

export async function runGetPixels(file: File): Promise<WasmImageData> {
  const inputFile = new Uint8Array(await file.arrayBuffer())
  return runWorker<WasmImageData>(GetPixelsWorker, { inputFile, inputType: getFileMimeType(file) }, undefined, 30_000)
}
