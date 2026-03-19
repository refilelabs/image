<script setup lang="ts">
import type { ImageActionResult } from '#image/utils/image_action_callback'
import type { RotateWorkerRequest } from '#image/workers/rotate.d'
import type { WorkerProgress } from '#image/workers/shared_types'
import { acceptList } from '#image/utils/file_types'
import { runWorker } from '#image/utils/run_worker'
import { parseWorkerError } from '#image/workers/shared_types'
import RotateWorker from '@/workers/rotate.ts?worker'

export interface RotateData {
  inputType: string
  degrees: number
  flipH: boolean
  flipV: boolean
  originalWidth: number
  originalHeight: number
}

export type RotateResult = ImageActionResult<RotateData>

const props = withDefaults(defineProps<{
  initFile?: File
  hint?: string
}>(), {
  hint: 'Any image file (i.e. png, jpg, jpeg, gif, webp, svg etc.)',
})

const emit = defineEmits<{
  rotate: [opts: RotateResult]
}>()

const toast = useToast()

const file = ref<File | undefined>(props.initFile)
const degrees = ref(0)
const flipH = ref(false)
const flipV = ref(false)
const progress = ref<WorkerProgress>()
const { decode, decoding } = useGetPixels()
const originalSize = ref<[number, number]>([0, 0])

const canvas = useTemplateRef('canvas')

const outputType = computed<keyof typeof outputFileEndings>(() => {
  const mime = file.value ? getFileMimeType(file.value) : 'image/png'
  return mime in outputFileEndings ? (mime as keyof typeof outputFileEndings) : 'image/png'
})

// CSS transform for preview — operation order matches WASM: flipH > flipV > rotate
// CSS applies right-to-left so scaleX is applied first, then scaleY, then rotate
const previewTransform = computed(() => {
  const fx = flipH.value ? -1 : 1
  const fv = flipV.value ? -1 : 1
  return `rotate(${degrees.value}deg) scaleY(${fv}) scaleX(${fx})`
})

function rotateLeft() {
  degrees.value = ((degrees.value - 90) + 360) % 360
}

function rotateRight() {
  degrees.value = (degrees.value + 90) % 360
}

function reset() {
  degrees.value = 0
  flipH.value = false
  flipV.value = false
}

async function tryLoadImage(f: File) {
  const res = await decode(f)
  if (!res)
    return
  const { width, height, pixels } = res

  originalSize.value = [width, height]

  await nextTick()
  const ctx = canvas.value?.getContext('2d')
  ctx?.putImageData(new ImageData(new Uint8ClampedArray(pixels) as Uint8ClampedArray<ArrayBuffer>, width, height), 0, 0)

  toast.add({ title: 'Image loaded', color: 'success', icon: 'heroicons:check-circle' })
}

watch(file, async (f) => {
  if (f) {
    originalSize.value = [0, 0]
    reset()
    await tryLoadImage(f)
  }
  else {
    originalSize.value = [0, 0]
    progress.value = undefined
  }
})

onMounted(() => {
  if (props.initFile)
    tryLoadImage(props.initFile)
})

async function download() {
  if (!file.value)
    return

  const arr = new Uint8Array(await file.value.arrayBuffer())
  const params: RotateWorkerRequest = {
    inputFile: arr,
    inputType: getFileMimeType(file.value),
    degrees: degrees.value,
    flipH: flipH.value,
    flipV: flipV.value,
  }

  try {
    const result = await runWorker<Uint8Array>(RotateWorker, params, p => progress.value = p)
    const ext = outputFileEndings[outputType.value]
    const name = removeFileExtension(file.value.name)
    const suffix = [
      flipH.value ? 'fliph' : '',
      flipV.value ? 'flipv' : '',
      degrees.value ? `rot${degrees.value}` : '',
    ].filter(Boolean).join('-') || 'original'

    emit('rotate', {
      file: new File([result as Uint8Array<ArrayBuffer>], `${name}-${suffix}.${ext}`, { type: outputType.value }),
      metrics: {
        inputType: getFileMimeType(file.value),
        degrees: degrees.value,
        flipH: flipH.value,
        flipV: flipV.value,
        originalWidth: originalSize.value[0],
        originalHeight: originalSize.value[1],
      },
    })
  }
  catch (e) {
    toast.add({ title: 'Error', icon: 'heroicons:exclamation-circle', description: parseWorkerError(e), color: 'error' })
    progress.value = undefined
  }
}
</script>

<template>
  <div class="w-full">
    <InputsMinimal
      v-model="file" :hint="hint" :accept="acceptList" :minimal="!!file"
      class="h-screen min-h-[30vh] max-h-[65vh]" :disable-input="!!file"
    >
      Choose File
      <template #file-preview>
        <div class="w-full h-full relative grid place-items-center overflow-hidden">
          <div v-if="decoding" class="absolute inset-0 z-10 grid place-items-center bg-default/60 backdrop-blur-sm">
            <div class="flex flex-col items-center gap-2">
              <UIcon name="heroicons:arrow-path" class="w-6 h-6 text-primary animate-spin" />
              <span class="text-xs text-muted">Decoding image...</span>
            </div>
          </div>
          <canvas
            v-show="originalSize[0]"
            ref="canvas"
            :width="originalSize[0]"
            :height="originalSize[1]"
            :style="{ transform: previewTransform }"
            class="max-w-full max-h-full object-contain transition-transform duration-200"
          />
          <span class="absolute top-2 left-2 px-2 py-1 rounded-md bg-accented/80 backdrop-blur-sm text-toned text-xs font-medium truncate max-w-[50%]">
            {{ file?.name }}
          </span>
          <span v-if="originalSize[0]" class="absolute bottom-2 left-2 px-2 py-1 rounded-md bg-accented/80 backdrop-blur-sm text-toned text-xs font-medium">
            {{ originalSize[0] }}×{{ originalSize[1] }}px
          </span>
          <div class="absolute top-2 right-2">
            <UButton
              label="Remove image" color="neutral" variant="subtle" size="sm"
              icon="heroicons:trash" @click.prevent="file = undefined"
            />
          </div>
        </div>
      </template>
    </InputsMinimal>

    <div v-if="file" class="mt-4 rounded-xl border border-default bg-elevated p-4 flex flex-col gap-4">
      <div class="flex flex-wrap items-center gap-3">
        <UButton color="neutral" variant="subtle" size="sm" icon="heroicons:arrow-uturn-left" @click="rotateLeft">
          Rotate Left
        </UButton>
        <UButton color="neutral" variant="subtle" size="sm" icon="heroicons:arrow-uturn-right" @click="rotateRight">
          Rotate Right
        </UButton>

        <UButton
          color="neutral" size="sm"
          :variant="flipH ? 'solid' : 'subtle'"
          icon="heroicons:arrows-right-left"
          @click="flipH = !flipH"
        >
          Flip H
        </UButton>
        <UButton
          color="neutral" size="sm"
          :variant="flipV ? 'solid' : 'subtle'"
          icon="heroicons:arrows-up-down"
          @click="flipV = !flipV"
        >
          Flip V
        </UButton>

        <UButton
          v-if="degrees !== 0 || flipH || flipV"
          color="neutral" variant="ghost" size="sm"
          icon="heroicons:x-mark"
          @click="reset"
        >
          Reset
        </UButton>

        <UButton class="ml-auto" trailing-icon="heroicons:arrow-down-tray" @click="download">
          Download
        </UButton>
      </div>

      <div class="flex items-center gap-2 text-xs text-muted">
        <span class="font-medium">{{ degrees }}°</span>
        <span v-if="flipH || flipV">·</span>
        <span v-if="flipH">Flipped H</span>
        <span v-if="flipH && flipV">+</span>
        <span v-if="flipV">Flipped V</span>
      </div>
    </div>

    <ImageWorkerProgress :progress="progress" />
  </div>
</template>
