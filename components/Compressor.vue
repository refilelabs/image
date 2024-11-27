<script setup lang="ts">
import type { WorkerProgress } from '#image/workers/shared_types'
import { acceptList } from '#image/utils/file_types'

const props = withDefaults(defineProps<{
  initFile?: File
  initOutputType?: string
  hint?: string
}>(), {
  hint: 'Any image file (i.e. png, jpg, jpeg, gif, webp, svg etc.)',
})

// const toast = useToast()

const file = ref<File | undefined>(props.initFile)

const progress = ref<WorkerProgress>()

const inputFileUrl = computed(() => file.value ? URL.createObjectURL(file.value) : undefined)

const x = ref<number>(0)

const formatBytes = (bytes: number, decimals = 2) => {
  if (bytes === 0)
    return '0 Bytes'
  const k = 1024
  const dm = decimals < 0 ? 0 : decimals
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return `${Number.parseFloat((bytes / k ** i).toFixed(dm))} ${sizes[i]}`
}

const imgElem = ref<HTMLImageElement | null>(null)

const { height, width, left, right } = useElementBounding(imgElem)

const comparisonWidth = computed(() => {
  if (x.value < left.value) {
    return '100%'
  }
  else if (x.value > right.value) {
    return '0%'
  }
  else {
    return `${98 - ((x.value - left.value) / (right.value - left.value)) * 100}%`
  }
})
</script>

<template>
  <div class="w-full">
    {{ x }}
    <InputsMinimal v-model="file" :hint="hint" :accept="acceptList" :minimal="!!file" class="h-screen max-h-[65vh]" :disable-input="!!file">
      Choose File
      <template #file-preview>
        <ImageComparisonTool v-model="x">
          <div class="w-full h-full notDraggable grid place-items-center">
            <img ref="imgElem" :src="inputFileUrl">
            <div
              class="absolute flex flex-row justify-end" :style="{
                width: `${width}px`,
                height: `${height}px`,
              }"
            >
              <div class="bg-pink-500 w-full h-full" :style="{ width: comparisonWidth }" />
            </div>
            <span class="absolute top-0 left-0 p-2 bg-[var(--ui-bg-accented)] text-[var(--ui-text-toned)] notDraggable">{{ file?.name }}</span>
            <span class="absolute bottom-0 left-0 p-2 bg-[var(--ui-bg-accented)] text-[var(--ui-text-toned)] notDraggable">{{ formatBytes(file?.size || 0) }}</span>
          </div>
        </ImageComparisonTool>
      </template>
    </InputsMinimal>

    <div class="flex flex-row items-end space-x-10 pt-3">
      <div class="grow">
        <UFormField label="Output Type" />
      </div>
      <UButton class="cursor-pointer">
        Start Compression
      </UButton>
    </div>

    <aside v-if="progress !== undefined" class="pt-6 w-full text-center">
      <UProgress v-model="progress.progress" />
      {{ progress.message }}
    </aside>
  </div>
</template>

<style scoped>
.notDraggable {
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
