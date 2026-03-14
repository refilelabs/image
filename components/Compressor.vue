<script setup lang="ts">
import type { ImageData } from '#image/wasm/pkg/bundler/refilelabs_image'
import type { WorkerProgress } from '#image/workers/shared_types'
import type { CompressionSettings } from './CompressionSettings.vue'
import { acceptList } from '#image/utils/file_types'
import { getPixels } from '#image/wasm/pkg/bundler/refilelabs_image'
import { breakpointsTailwind } from '@vueuse/core'

export interface CompressionData {
  inputType: string
  outputType: string
  savings: number
  quality: number
}

export type CompressionResult = ImageActionResult<CompressionData>

const props = withDefaults(defineProps<{
  initFile?: File
  initOutputType?: string
  hint?: string
}>(), {
  hint: 'Any image file (i.e. png, jpg, jpeg, gif, webp, svg etc.)',
})

const emit = defineEmits<{
  compress: [opts: CompressionResult]
}>()

const toast = useToast()

const file = ref<File | undefined>(props.initFile)

const progress = ref<WorkerProgress>()

const x = ref<number>(0)

const container = useTemplateRef('container')

const { width: maxWidth, height: maxHeight } = useElementBounding(container)

const canvas = useTemplateRef('canvas')

const { height, width, left, right } = useElementBounding(canvas)

const comparisonWidth = computed(() => {
  if (x.value < left.value) {
    return '0%'
  }
  else if (x.value > right.value) {
    return '100%'
  }
  else {
    return `${((x.value - left.value) / (right.value - left.value)) * 100}%`
  }
})

const imageData = reactive<Partial<Omit<ImageData, 'aspect_ratio' | 'color_depth'>>>({
  width: undefined,
  height: undefined,
  pixels: undefined,
})

const compressionSettings = reactive<CompressionSettings>({
  quality: 0.7,
  type: 'image/webp',
})

async function createBlob(canvas: HTMLCanvasElement, type: string, quality: number): Promise<Blob> {
  return new Promise((resolve) => {
    canvas.toBlob((blob) => {
      if (blob) {
        resolve(blob)
      }
      else {
        throw new Error('Failed to create blob')
      }
    }, type, quality)
  })
}

const compressedCanvas = useTemplateRef('compressedCanvas')

const compressedSize = ref<number>()

async function drawCompressedImage(settings: CompressionSettings) {
  if (file.value && canvas.value && compressedCanvas.value) {
    const ctx = compressedCanvas.value.getContext('2d')

    if (ctx) {
      ctx.drawImage(canvas.value, 0, 0)

      const blob = await createBlob(compressedCanvas.value, settings.type, settings.quality)

      compressedSize.value = blob.size

      const url = URL.createObjectURL(blob)

      const img = new Image()

      img.onload = () => {
        ctx.drawImage(img, 0, 0)
      }

      img.src = url
    }
  }
}

const tryLoadImage = async (file: File) => {
  const arraybuffer = await file.arrayBuffer()
  const arr = new Uint8Array(arraybuffer)

  const res = getPixels(arr, getFileMimeType(file))

  const { width, height, pixels: rawPixels } = res

  imageData.width = width
  imageData.height = height
  imageData.pixels = rawPixels

  await nextTick()

  const pixels = new Uint8ClampedArray(rawPixels)

  const ctx = canvas.value?.getContext('2d')

  if (ctx) {
    const imageData = new ImageData(pixels, width, height)
    ctx.putImageData(imageData, 0, 0)
    toast.add({ title: 'Success', description: 'Image loaded successfully', color: 'success', icon: 'heroicons:check-circle' })

    drawCompressedImage(compressionSettings)
  }
}

watch(file, async (newFile) => {
  if (newFile) {
    tryLoadImage(newFile)
  }
  else {
    imageData.width = undefined
    imageData.height = undefined
    imageData.pixels = undefined
  }
})

watchThrottled(compressionSettings, async (settings) => {
  drawCompressedImage(settings)
}, {
  throttle: 500,
})

watch(compressionSettings, () => {
  if (file.value && canvas.value && compressedCanvas.value) {
    compressedSize.value = undefined
  }
}, {
  immediate: true,
})

async function compress() {
  const blob = await createBlob(compressedCanvas.value!, compressionSettings.type, compressionSettings.quality)
  const arrayBuffer = await blob.arrayBuffer()

  const fileNameWithoutExtension = file.value?.name.replace(/\.[^/.]+$/, '')
  const extension = compressionSettings.type.split('/')[1]

  const returnedFile = new File([new Uint8Array(arrayBuffer)], `${fileNameWithoutExtension}.${extension}`, { type: compressionSettings.type })

  emit('compress', {
    file: returnedFile,
    metrics: {
      inputType: getFileMimeType(file.value as File),
      outputType: compressionSettings.type,
      savings: (1 - (compressedSize.value! / (file.value?.size || 1))) * 100,
      quality: compressionSettings.quality,
    },
  })
}

const breakpoints = useBreakpoints(breakpointsTailwind)

const smallScreen = breakpoints.smallerOrEqual('sm')

onMounted(() => {
  if (file.value) {
    tryLoadImage(file.value)
  }
})
</script>

<template>
  <div class="w-full">
    <InputsMinimal v-model="file" :hint="hint" :accept="acceptList" :minimal="!!file" class="h-screen min-h-[30vh] max-h-[65vh]" :disable-input="!!file">
      Choose File
      <template #file-preview>
        <div ref="container" class="w-full h-full relative">
          <ImageComparisonTool v-model="x">
            <div class="w-full h-full notDraggable grid place-items-center">
              <canvas
                v-show="file && imageData.width && imageData.height" ref="canvas" :width="imageData.width || 0" :height="imageData.height || 0" class="max-w-full max-h-full" :style="{
                  maxWidth: `${maxWidth}px`,
                  maxHeight: `${maxHeight}px`,
                }"
              />
              <div
                class="absolute flex flex-row justify-end" :style="{
                  width: `${width}px`,
                  height: `${height}px`,
                }"
              >
                <canvas
                  ref="compressedCanvas" :width="imageData.width || 0" :height="imageData.height || 0" class="w-full h-full" :style="{
                    'clip-path': `inset(0 0 0 ${comparisonWidth})`,
                    'max-height': `${maxHeight}px`,
                    'max-width': `${maxWidth}px`,
                  }"
                />
              </div>
              <span class="absolute top-2 left-2 px-2 py-1 rounded-md bg-(--ui-bg-accented)/80 backdrop-blur-sm text-toned text-xs font-medium truncate max-w-[50%]">{{ file?.name }}</span>
              <span class="absolute bottom-2 left-2 px-2 py-1 rounded-md bg-(--ui-bg-accented)/80 backdrop-blur-sm text-toned text-xs font-medium">{{ formatBytes(file?.size || 0) }}</span>
              <div class="absolute bottom-2 right-2 px-2 py-1 rounded-md bg-(--ui-bg-accented)/80 backdrop-blur-sm text-toned text-xs font-medium flex flex-row items-center gap-1.5">
                <template v-if="compressedSize === undefined">
                  <UIcon name="heroicons:arrow-path" class="animate-spin h-3 w-3" />
                  <span>Compressing...</span>
                </template>
                <template v-else>
                  <UIcon name="heroicons:check-circle" class="text-success h-3 w-3" />
                  <span>{{ formatBytes(compressedSize) }}</span>
                </template>
              </div>
            </div>
          </ImageComparisonTool>

          <div class="absolute top-0 right-0 p-2">
            <UFieldGroup :orientation="smallScreen ? 'vertical' : 'horizontal'">
              <UButton label="Remove image" color="neutral" variant="subtle" icon="heroicons:trash" @click.prevent="file = undefined" />
              <ImageCompressionSettings v-model:quality="compressionSettings.quality" v-model:type="compressionSettings.type" />
            </UFieldGroup>
          </div>
        </div>
      </template>
    </InputsMinimal>

    <div v-if="file" class="mt-4 rounded-xl border border-default bg-elevated p-4 flex items-center justify-between gap-4">
      <div v-if="compressedSize !== undefined" class="flex items-center gap-2 text-sm">
        <span class="text-muted">{{ formatBytes(file?.size || 0) }}</span>
        <UIcon name="heroicons:arrow-right-16-solid" class="h-3.5 w-3.5 text-dimmed" />
        <span class="font-semibold text-success">{{ formatBytes(compressedSize) }}</span>
        <span class="text-xs text-muted">({{ ((1 - compressedSize / (file?.size || 1)) * 100).toFixed(0) }}% smaller)</span>
      </div>
      <div v-else class="flex items-center gap-2 text-sm text-muted">
        <UIcon name="heroicons:arrow-path" class="animate-spin h-4 w-4" />
        <span>Generating preview...</span>
      </div>
      <UButton trailing-icon="heroicons:arrow-down-tray" :disabled="!file || !imageData.pixels" @click="compress">
        Download Compressed
      </UButton>
    </div>

    <ImageWorkerProgress :progress="progress" />
  </div>
</template>

<style scoped>
.notDraggable * {
  user-drag: none;
  -webkit-user-drag: none;
  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  -ms-user-select: none;
  pointer-events: none;
  touch-action: none;
}
</style>
