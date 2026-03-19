<script setup lang="ts">
import type { ImageData } from '#image/wasm/pkg/web/refilelabs_image'
import { acceptList } from '#image/utils/file_types'

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
const { decode, decoding } = useGetPixels()

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
  const start = performance.now()

  const res = await decode(file)
  if (!res)
    return

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
          <div v-if="decoding" class="absolute inset-0 z-10 grid place-items-center bg-(--ui-bg)/60 backdrop-blur-sm">
            <div class="flex flex-col items-center gap-2">
              <UIcon name="heroicons:arrow-path" class="w-6 h-6 text-primary animate-spin" />
              <span class="text-xs text-muted">Decoding image...</span>
            </div>
          </div>
          <canvas
            v-show="file && imageData.width && imageData.height" ref="canvas" :width="imageData.width || 0" :height="imageData.height || 0" :style="{
              maxWidth: `${maxWidth}px`,
              maxHeight: `${maxHeight}px`,
            }"
          />
          <span class="absolute top-2 left-2 px-2 py-1 rounded-md bg-(--ui-bg-accented)/80 backdrop-blur-sm text-toned text-xs font-medium truncate max-w-[50%]">{{ file?.name }}</span>
          <span class="absolute bottom-2 left-2 px-2 py-1 rounded-md bg-(--ui-bg-accented)/80 backdrop-blur-sm text-toned text-xs font-medium">{{ formatBytes(file?.size || 0) }}</span>
          <div class="absolute bottom-2 right-2 px-2 py-1 rounded-md bg-(--ui-bg-accented)/80 backdrop-blur-sm text-toned text-xs font-medium grid grid-cols-2 gap-x-3 gap-y-0.5">
            <span class="text-dimmed">Dimensions</span> <span>{{ imageData.width }}×{{ imageData.height }}</span>
            <span class="text-dimmed">Aspect ratio</span> <span>{{ imageData.aspect_ratio }}</span>
            <span class="text-dimmed">Color depth</span> <span>{{ imageData.color_depth }} bits</span>
          </div>
          <span class="absolute top-2 right-2">
            <UButton variant="subtle" color="neutral" size="sm" label="Clear" icon="heroicons:trash" @click.prevent="file = undefined" />
          </span>
        </div>
      </template>
    </InputsMinimal>
  </div>
</template>
