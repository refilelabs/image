<script setup lang="ts">
import type { CropRect } from '#image/composables/useCropDrag'
import type { ImageActionResult } from '#image/utils/image_action_callback'
import type { CropWorkerRequest } from '#image/workers/crop.d'
import type { WorkerProgress } from '#image/workers/shared_types'
import { getHandles, HANDLE_SIZE, useCropDrag } from '#image/composables/useCropDrag'
import { acceptList } from '#image/utils/file_types'
import { runWorker } from '#image/utils/run_worker'
import { parseWorkerError } from '#image/workers/shared_types'
import CropWorker from '@/workers/crop.ts?worker'

export interface CropData {
  inputType: string
  originalWidth: number
  originalHeight: number
  cropX: number
  cropY: number
  cropWidth: number
  cropHeight: number
}

export type CropResult = ImageActionResult<CropData>

const props = withDefaults(defineProps<{
  initFile?: File
  hint?: string
}>(), {
  hint: 'Any image file (i.e. png, jpg, jpeg, gif, webp, svg etc.)',
})

const emit = defineEmits<{
  crop: [opts: CropResult]
}>()

const toast = useToast()

const file = ref<File | undefined>(props.initFile)
const originalSize = ref<[number, number]>([0, 0])
const progress = ref<WorkerProgress>()
const { decode, decoding } = useGetPixels()
const crop = ref<CropRect>()
const aspectLock = ref('free')

const outputType = computed<keyof typeof outputFileEndings>(() => {
  const mime = file.value ? getFileMimeType(file.value) : 'image/png'
  return mime in outputFileEndings ? (mime as keyof typeof outputFileEndings) : 'image/png'
})

// ---- canvas refs ----

const sourceCanvas = useTemplateRef('sourceCanvas')
const displayCanvas = useTemplateRef('displayCanvas')
const container = useTemplateRef('container')
const { width: containerWidth, height: containerHeight } = useElementBounding(container)

// Scale between display canvas pixel size and image pixel size
const displayScale = ref({ x: 1, y: 1 })
const displayWidth = ref(0)
const displayHeight = ref(0)

// ---- drag interaction (extracted composable) ----

const { onMouseDown, onTouchStart, onTouchMove, onTouchEnd, selectAll, onNumericInput } = useCropDrag({
  crop,
  originalSize,
  displayScale,
  displayCanvas,
  aspectLock,
})

// ---- canvas drawing ----

function drawOverlay() {
  if (!displayCanvas.value || !sourceCanvas.value || !crop.value)
    return
  if (!originalSize.value[0] || !originalSize.value[1])
    return

  const canvas = displayCanvas.value
  const ctx = canvas.getContext('2d')
  if (!ctx)
    return

  const [imgW, imgH] = originalSize.value
  const sx = canvas.width / imgW
  const sy = canvas.height / imgH
  displayScale.value = { x: sx, y: sy }

  const { x, y, w, h } = crop.value
  const dx = x * sx
  const dy = y * sy
  const dw = w * sx
  const dh = h * sy

  drawImage(ctx, canvas)
  drawDimOverlay(ctx, canvas, dx, dy, dw, dh)
  drawGrid(ctx, dx, dy, dw, dh)
  drawHandles(ctx, dx, dy, dw, dh)
}

function drawImage(ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement) {
  ctx.clearRect(0, 0, canvas.width, canvas.height)
  ctx.drawImage(sourceCanvas.value!, 0, 0, canvas.width, canvas.height)
}

function drawDimOverlay(ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement, dx: number, dy: number, dw: number, dh: number) {
  // Darken everything outside the crop rect
  ctx.fillStyle = 'rgba(0,0,0,0.5)'
  ctx.fillRect(0, 0, canvas.width, canvas.height)
  // Restore crop area
  ctx.clearRect(dx, dy, dw, dh)
  ctx.drawImage(sourceCanvas.value!, crop.value!.x, crop.value!.y, crop.value!.w, crop.value!.h, dx, dy, dw, dh)
  // Dashed border
  ctx.strokeStyle = 'rgba(255,255,255,0.9)'
  ctx.lineWidth = 1.5
  ctx.setLineDash([5, 3])
  ctx.strokeRect(dx, dy, dw, dh)
  ctx.setLineDash([])
}

function drawDashedLine(ctx: CanvasRenderingContext2D, x1: number, y1: number, x2: number, y2: number, dash: number) {
  ctx.setLineDash([dash, dash])

  ctx.lineDashOffset = 0
  ctx.strokeStyle = 'rgba(0,0,0,0.45)'
  ctx.beginPath()
  ctx.moveTo(x1, y1)
  ctx.lineTo(x2, y2)
  ctx.stroke()

  ctx.lineDashOffset = dash
  ctx.strokeStyle = 'rgba(255,255,255,0.7)'
  ctx.beginPath()
  ctx.moveTo(x1, y1)
  ctx.lineTo(x2, y2)
  ctx.stroke()

  ctx.setLineDash([])
  ctx.lineDashOffset = 0
}

function drawGrid(ctx: CanvasRenderingContext2D, dx: number, dy: number, dw: number, dh: number) {
  // Rule-of-thirds grid with alternating dark/light dashes — visible on any background
  ctx.lineWidth = 1
  const dash = 5
  for (let i = 1; i <= 2; i++) {
    drawDashedLine(ctx, dx + (dw * i) / 3, dy, dx + (dw * i) / 3, dy + dh, dash) // vertical
    drawDashedLine(ctx, dx, dy + (dh * i) / 3, dx + dw, dy + (dh * i) / 3, dash) // horizontal
  }
}

function drawHandles(ctx: CanvasRenderingContext2D, dx: number, dy: number, dw: number, dh: number) {
  const hs = HANDLE_SIZE
  const primaryColor = getComputedStyle(document.documentElement).getPropertyValue('--ui-primary').trim()
  for (const [key, pt] of Object.entries(getHandles({ x: dx, y: dy, w: dw, h: dh }))) {
    // Inset handles so they stay fully inside the crop border
    let hx = pt.x - hs / 2
    let hy = pt.y - hs / 2
    if (key.includes('W'))
      hx = dx
    if (key.includes('E'))
      hx = dx + dw - hs
    if (key.includes('N'))
      hy = dy
    if (key.includes('S'))
      hy = dy + dh - hs

    ctx.shadowColor = 'rgba(0,0,0,0.5)'
    ctx.shadowBlur = 3
    ctx.fillStyle = primaryColor || 'violet'
    ctx.fillRect(hx, hy, hs, hs)
    ctx.shadowBlur = 0
    ctx.strokeStyle = 'rgba(255,255,255,0.85)'
    ctx.lineWidth = 1.5
    ctx.strokeRect(hx, hy, hs, hs)
  }
}

watch(crop, () => nextTick(drawOverlay), { deep: true })

watch([containerWidth, containerHeight], () => {
  if (!originalSize.value[0])
    return
  const [imgW, imgH] = originalSize.value
  const ratio = imgW / imgH
  if (containerWidth.value / containerHeight.value > ratio) {
    displayHeight.value = Math.round(containerHeight.value)
    displayWidth.value = Math.round(containerHeight.value * ratio)
  }
  else {
    displayWidth.value = Math.round(containerWidth.value)
    displayHeight.value = Math.round(containerWidth.value / ratio)
  }
  nextTick(drawOverlay)
})

// ---- aspect lock ----

const aspectOptions = [
  { label: 'Free', value: 'free' },
  { label: '1:1', value: '1:1' },
  { label: '16:9', value: '16:9' },
  { label: '4:3', value: '4:3' },
  { label: '3:2', value: '3:2' },
]

const aspectRatioMap: Record<string, number> = { '1:1': 1, '16:9': 16 / 9, '4:3': 4 / 3, '3:2': 3 / 2 }

watch(aspectLock, () => {
  if (!crop.value || aspectLock.value === 'free')
    return
  const ratio = aspectRatioMap[aspectLock.value]
  if (!ratio)
    return
  const [imgW, imgH] = originalSize.value
  const clampedH = Math.min(Math.round(crop.value.w / ratio), imgH - crop.value.y)
  crop.value = {
    ...crop.value,
    w: Math.min(Math.round(clampedH * ratio), imgW - crop.value.x),
    h: clampedH,
  }
})

// ---- image loading ----

async function tryLoadImage(f: File) {
  const res = await decode(f)
  if (!res)
    return

  const { width, height, pixels } = res

  originalSize.value = [width, height]

  await nextTick()
  const ctx = sourceCanvas.value?.getContext('2d')
  ctx?.putImageData(new ImageData(new Uint8ClampedArray(pixels) as Uint8ClampedArray<ArrayBuffer>, width, height), 0, 0)

  crop.value = { x: 0, y: 0, w: width, h: height }
  await nextTick()
  drawOverlay()

  toast.add({ title: 'Image loaded', color: 'success', icon: 'heroicons:check-circle' })
}

watch(file, async (f) => {
  if (f) {
    await tryLoadImage(f)
  }
  else {
    originalSize.value = [0, 0]
    crop.value = undefined
    progress.value = undefined
  }
})

onMounted(() => {
  if (file.value)
    tryLoadImage(file.value)
})

// ---- download ----

async function download() {
  if (!file.value || !crop.value)
    return
  const { x, y, w, h } = crop.value
  if (w < 1 || h < 1)
    return

  const arr = new Uint8Array(await file.value.arrayBuffer())
  const params: CropWorkerRequest = { inputFile: arr, inputType: getFileMimeType(file.value), x, y, width: w, height: h }

  try {
    const result = await runWorker<Uint8Array>(CropWorker, params, p => progress.value = p)
    const ext = outputFileEndings[outputType.value]
    const name = removeFileExtension(file.value.name)

    emit('crop', {
      file: new File([result as Uint8Array<ArrayBuffer>], `${name}-cropped-${w}x${h}.${ext}`, { type: outputType.value }),
      metrics: {
        inputType: getFileMimeType(file.value),
        originalWidth: originalSize.value[0],
        originalHeight: originalSize.value[1],
        cropX: x,
        cropY: y,
        cropWidth: w,
        cropHeight: h,
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
    <canvas ref="sourceCanvas" :width="originalSize[0]" :height="originalSize[1]" class="hidden" />

    <InputsMinimal
      v-model="file" :hint="hint" :accept="acceptList" :minimal="!!file"
      class="h-screen min-h-[30vh] max-h-[65vh]" :disable-input="!!file"
    >
      Choose File
      <template #file-preview>
        <div ref="container" class="w-full h-full relative grid place-items-center overflow-hidden">
          <div v-if="decoding" class="absolute inset-0 z-10 grid place-items-center bg-(--ui-bg)/60 backdrop-blur-sm">
            <div class="flex flex-col items-center gap-2">
              <UIcon name="heroicons:arrow-path" class="w-6 h-6 text-primary animate-spin" />
              <span class="text-xs text-muted">Decoding image...</span>
            </div>
          </div>
          <canvas
            v-show="file && originalSize[0]"
            ref="displayCanvas"
            :width="displayWidth || originalSize[0]"
            :height="displayHeight || originalSize[1]"
            class="max-w-full max-h-full cursor-crosshair touch-none"
            :style="{ maxWidth: `${containerWidth}px`, maxHeight: `${containerHeight}px` }"
            @mousedown="onMouseDown"
            @touchstart.prevent="onTouchStart"
            @touchmove.prevent="onTouchMove"
            @touchend="onTouchEnd"
          />
          <span class="absolute top-2 left-2 px-2 py-1 rounded-md bg-(--ui-bg-accented)/80 backdrop-blur-sm text-toned text-xs font-medium truncate max-w-[50%]">
            {{ file?.name }}
          </span>
          <span class="absolute bottom-2 left-2 px-2 py-1 rounded-md bg-(--ui-bg-accented)/80 backdrop-blur-sm text-toned text-xs font-medium">
            {{ originalSize[0] }}×{{ originalSize[1] }}px
          </span>
          <span v-if="crop" class="absolute bottom-2 right-2 px-2 py-1 rounded-md bg-(--ui-bg-accented)/80 backdrop-blur-sm text-toned text-xs font-medium">
            {{ crop.w }}×{{ crop.h }}px
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

    <div v-if="file && crop" class="mt-4 rounded-xl border border-default bg-elevated p-4 flex flex-col gap-4">
      <div class="flex flex-wrap items-center gap-3">
        <div v-for="(field, label) in { X: 'x', Y: 'y', W: 'w', H: 'h' } as Record<string, keyof CropRect>" :key="field" class="flex items-center gap-2">
          <label class="text-xs text-muted font-medium w-4">{{ label }}</label>
          <UInput
            :model-value="crop[field]" type="number" size="sm" class="w-20"
            @update:model-value="onNumericInput(field, String($event))"
          />
        </div>

        <div class="flex items-center gap-2">
          <label class="text-xs text-muted font-medium">Aspect</label>
          <USelectMenu
            v-model="aspectLock"
            :items="aspectOptions"
            value-key="value"
            label-key="label"
            size="sm"
            class="w-24"
          />
        </div>

        <UButton color="neutral" variant="subtle" size="sm" icon="heroicons:arrows-pointing-out" @click="selectAll">
          Select All
        </UButton>

        <UButton class="cursor-pointer ml-auto" trailing-icon="heroicons:arrow-down-tray" :disabled="!crop || crop.w < 1 || crop.h < 1" @click="download">
          Download
        </UButton>
      </div>
    </div>

    <ImageWorkerProgress :progress="progress" />
  </div>
</template>
