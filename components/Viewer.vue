<script setup lang="ts">
import { acceptList } from '#image/utils/file_types'
import init, { getPixels, type ImageData } from '#image/wasm/pkg/refilelabs_image'

export interface ViewerData {
  width: number
  height: number
  aspectRatio: number
  colorDepth: number
  inputType: string
  duration: number
}

const props = withDefaults(defineProps<{
  initFile?: File
  initOutputType?: string
  hint?: string
}>(), {
  hint: 'Any image file (i.e. png, jpg, jpeg, gif, webp, svg etc.)',
})

const emit = defineEmits<{
  view: [opts: ViewerData]
}>()

const toast = useToast()

const file = ref<File | undefined>(props.initFile)

const container = useTemplateRef('container')

const { width: maxWidth, height: maxHeight } = useElementBounding(container)

const canvas = useTemplateRef('canvas')

const imageData = reactive<Partial<Omit<ImageData, 'pixels'>>>({
  width: undefined,
  height: undefined,
  aspect_ratio: undefined,
  color_depth: undefined,
})

const tryDrawFile = async (file: File) => {
  await init()

  const start = performance.now()

  const arraybuffer = await file.arrayBuffer()
  const arr = new Uint8Array(arraybuffer)

  const res = getPixels(arr, getFileMimeType(file))

  const { width, height, aspect_ratio, color_depth } = res

  imageData.width = width
  imageData.height = height
  imageData.aspect_ratio = aspect_ratio
  imageData.color_depth = color_depth

  await nextTick()

  const ctx = canvas.value?.getContext('2d')

  if (ctx) {
    const pixels = new Uint8ClampedArray(res.pixels)
    const imageData = new ImageData(pixels, width, height)
    ctx.putImageData(imageData, 0, 0)
    toast.add({ title: 'Success', description: 'Image loaded successfully', color: 'success', icon: 'heroicons:check-circle' })
  }

  emit('view', {
    width,
    height,
    aspectRatio: aspect_ratio,
    colorDepth: color_depth,
    inputType: file.type,
    duration: performance.now() - start,
  })
}

watch(file, async (newFile) => {
  if (newFile) {
    await tryDrawFile(newFile)
  }
  else {
    imageData.width = undefined
    imageData.height = undefined
    imageData.aspect_ratio = undefined
    imageData.color_depth = undefined
  }
})

onMounted(() => {
  if (file.value) {
    tryDrawFile(file.value)
  }
})
</script>

<template>
  <div class="w-full">
    <InputsMinimal v-model="file" :hint="hint" :accept="acceptList" :minimal="!!file" class="h-screen max-h-[65vh]" :disable-input="!!file">
      Choose File
      <template #file-preview>
        <div ref="container" class="w-full h-full grid place-items-center relative">
          <canvas
            v-show="file && imageData.width && imageData.height" ref="canvas" :width="imageData.width || 0" :height="imageData.height || 0" :style="{
              maxWidth: `${maxWidth}px`,
              maxHeight: `${maxHeight}px`,
            }"
          />
          <span class="absolute top-0 left-0 p-2 bg-[var(--ui-bg-accented)] text-[var(--ui-text-toned)]">{{ file?.name }}</span>
          <span class="absolute bottom-0 left-0 p-2 bg-[var(--ui-bg-accented)] text-[var(--ui-text-toned)]">{{ formatBytes(file?.size || 0) }}</span>
          <div class="absolute bottom-0 right-0 p-2 bg-[var(--ui-bg-accented)] text-[var(--ui-text-toned)] grid grid-cols-2">
            <span>Dimensions:</span> <span>{{ imageData.width }}x{{ imageData.height }}</span>
            <span>Aspect Ratio:</span> <span>{{ imageData.aspect_ratio }}</span>
            <span>Color Depth:</span> <span>{{ imageData.color_depth }} bits</span>
          </div>
          <span class="absolute top-0 right-0 bg-[var(--ui-bg-accented)] text-[var(--ui-text-toned)]">
            <UButton variant="link" size="xl" color="neutral" label="Clear" icon="heroicons:trash" @click.prevent="file = undefined" />
          </span>
        </div>
      </template>
    </InputsMinimal>
  </div>
</template>
