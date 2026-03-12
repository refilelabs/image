<script setup lang="ts">
import type { ResizeWorkerRequest } from '#image/workers/resize.d'
import type { WorkerProgress } from '#image/workers/shared_types'
import { acceptList } from '#image/utils/file_types'
import { runWorker } from '#image/utils/run_worker'
import { getPixels } from '#image/wasm/pkg/bundler/refilelabs_image'
import { parseWorkerError } from '#image/workers/shared_types'
import ResizeWorker from '@/workers/resize.ts?worker'

export interface ResizeData {
  inputType: string
  originalWidth: number
  originalHeight: number
  targetWidth: number
  targetHeight: number
}

export type ResizeResult = ImageActionResult<ResizeData>

const props = withDefaults(defineProps<{
  initFile?: File
  hint?: string
}>(), {
  hint: 'Any image file (i.e. png, jpg, jpeg, gif, webp, svg etc.)',
})

const emit = defineEmits<{
  resize: [opts: ResizeResult]
}>()

const toast = useToast()

const file = ref<File | undefined>(props.initFile)

const originalSize = ref<[number, number]>([0, 0])
const size = ref<[number, number]>([0, 0])
const progress = ref<WorkerProgress>()

const aspectRatio = computed(() =>
  originalSize.value[1] > 0 ? originalSize.value[0] / originalSize.value[1] : 1,
)

// Output type matches input; fall back to PNG for formats that can't be encoded (e.g. SVG, RAW)
const outputType = computed<keyof typeof outputFileEndings>(() => {
  const mime = file.value ? getFileMimeType(file.value) : 'image/png'
  return mime in outputFileEndings ? (mime as keyof typeof outputFileEndings) : 'image/png'
})

const sourceCanvas = useTemplateRef('sourceCanvas')
const previewCanvas = useTemplateRef('previewCanvas')
const container = useTemplateRef('container')

const { width: containerWidth, height: containerHeight } = useElementBounding(container)

function drawPreview() {
  if (!sourceCanvas.value || !previewCanvas.value || !size.value[0] || !size.value[1])
    return
  if (!sourceCanvas.value.width || !sourceCanvas.value.height)
    return
  const ctx = previewCanvas.value.getContext('2d')
  if (!ctx)
    return
  ctx.clearRect(0, 0, previewCanvas.value.width, previewCanvas.value.height)
  ctx.drawImage(sourceCanvas.value, 0, 0, size.value[0], size.value[1])
}

watch(size, () => drawPreview(), { flush: 'post' })

async function tryLoadImage(f: File) {
  const arr = new Uint8Array(await f.arrayBuffer())
  const res = getPixels(arr, getFileMimeType(f))
  const { width, height, pixels: rawPixels } = res

  originalSize.value = [width, height]
  size.value = [width, height]

  await nextTick()

  const ctx = sourceCanvas.value?.getContext('2d')
  if (ctx) {
    ctx.putImageData(new ImageData(new Uint8ClampedArray(rawPixels), width, height), 0, 0)
    drawPreview()
    toast.add({ title: 'Image loaded', color: 'success', icon: 'heroicons:check-circle' })
  }
}

watch(file, async (f) => {
  if (f) {
    await tryLoadImage(f)
  }
  else {
    originalSize.value = [0, 0]
    size.value = [0, 0]
    progress.value = undefined
  }
})

function doResize(arr: Uint8Array, inputType: MimeTypes, width: number, height: number): Promise<Uint8Array> {
  const params: ResizeWorkerRequest = { inputFile: arr, inputType, width, height }
  return runWorker<Uint8Array>(ResizeWorker, params, (p) => {
    progress.value = p
  })
}

async function download() {
  if (!file.value || !size.value[0] || !size.value[1])
    return

  const [targetWidth, targetHeight] = size.value

  const arraybuffer = await file.value.arrayBuffer()
  const arr = new Uint8Array(arraybuffer)

  try {
    const result = await doResize(arr, getFileMimeType(file.value), targetWidth, targetHeight)

    const ext = outputFileEndings[outputType.value]
    const name = removeFileExtension(file.value.name)
    const outputFile = new File([result as Uint8Array<ArrayBuffer>], `${name}-${targetWidth}x${targetHeight}.${ext}`, { type: outputType.value })

    emit('resize', {
      file: outputFile,
      metrics: {
        inputType: getFileMimeType(file.value),
        originalWidth: originalSize.value[0],
        originalHeight: originalSize.value[1],
        targetWidth: size.value[0],
        targetHeight: size.value[1],
      },
    })
  }
  catch (e) {
    const error = parseWorkerError(e)
    toast.add({
      title: 'Error',
      icon: 'i-heroicons-exclamation-circle',
      description: error,
      color: 'error',
    })
    progress.value = undefined
  }
}

onMounted(() => {
  if (file.value)
    tryLoadImage(file.value)
})
</script>

<template>
  <div class="w-full">
    <canvas ref="sourceCanvas" :width="originalSize[0]" :height="originalSize[1]" class="hidden" />

    <InputsMinimal
      v-model="file" :hint="hint" :accept="acceptList" :minimal="!!file"
      class="h-screen min-h-[30vh] max-h-[65vh]" :disable-input="!!file"
    >
      Choose File
      <template #file-preview>
        <div ref="container" class="w-full h-full relative grid place-items-center">
          <canvas
            v-show="file && size[0] && size[1]"
            ref="previewCanvas"
            :width="size[0] || 0"
            :height="size[1] || 0"
            class="max-w-full max-h-full"
            :style="{ maxWidth: `${containerWidth}px`, maxHeight: `${containerHeight}px` }"
          />
          <span class="absolute top-0 left-0 p-2 bg-(--ui-bg-accented) text-(--ui-text-toned)">
            {{ file?.name }}
          </span>
          <span class="absolute bottom-0 left-0 p-2 bg-(--ui-bg-accented) text-(--ui-text-toned)">
            {{ originalSize[0] }} × {{ originalSize[1] }}px
          </span>
          <span class="absolute bottom-0 right-0 p-2 bg-(--ui-bg-accented) text-(--ui-text-toned)">
            {{ size[0] }} × {{ size[1] }}px
          </span>
          <div class="absolute top-0 right-0 p-2">
            <UButton
              label="Remove image" color="neutral" variant="subtle"
              icon="heroicons:trash" @click.prevent="file = undefined"
            />
          </div>
        </div>
      </template>
    </InputsMinimal>

    <div v-if="file" class="flex flex-col sm:flex-row items-start sm:items-end gap-4 pt-4">
      <InputsSize
        v-model:size="size"
        :aspect-ratio="aspectRatio"
        :source-dimensions="[originalSize[0], originalSize[1]]"
      />

      <UButton
        class="cursor-pointer sm:ml-auto"
        trailing-icon="heroicons:arrow-down-tray"
        :disabled="!file || !size[0] || !size[1]"
        @click="download"
      >
        Download
      </UButton>
    </div>

    <ImageWorkerProgress :progress="progress" />
  </div>
</template>
