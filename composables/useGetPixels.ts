import type { ImageData as WasmImageData } from '#image/wasm/pkg/web/refilelabs_image'
import { runGetPixels } from '#image/utils/run_get_pixels'
import { parseWorkerError } from '#image/workers/shared_types'

export function useGetPixels() {
  const decoding = ref(false)
  const toast = useToast()

  async function decode(file: File): Promise<WasmImageData | null> {
    decoding.value = true
    try {
      return await runGetPixels(file)
    }
    catch (e) {
      toast.add({ title: 'Failed to decode image', description: parseWorkerError(e), color: 'error', icon: 'heroicons:exclamation-circle' })
      return null
    }
    finally {
      decoding.value = false
    }
  }

  return { decode, decoding }
}
