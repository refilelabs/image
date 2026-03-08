<script setup lang="ts">
import type { Metadata } from '#image/wasm/pkg/refilelabs_image'
import type { MetadataWorkerRequest } from '#image/workers/metadata.d'
import type { WorkerProgress } from '#image/workers/shared_types'
import type { AlertProps } from '@nuxt/ui'
import { parseWorkerError } from '#image/workers/shared_types'
import { runWorker } from '#image/utils/run_worker'
import MetadataWorker from '@/workers/metadata.ts?worker'

export interface MetadataExtractData {
  inputType: string
  duration: number
}

const props = withDefaults(defineProps<{
  initFile?: File
  hint?: string
}>(), {
  hint: 'Any image file (i.e. png, jpg, jpeg, gif, webp, svg etc.)',
})

const emit = defineEmits<{
  extract: [opts: MetadataExtractData]
}>()

const toast = useToast()

const file = ref<File | undefined>(props.initFile)

const metadata = ref<Metadata>()

type TabularMetadata = {
  property: string
  value: string
}[]

const search = ref('')

const tabularMetadata = computed(() => {
  const data: TabularMetadata = []

  const otherKey = 'other'

  if (metadata.value) {
    for (const [key, value] of Object.entries(metadata.value).filter(([key, value]) => key !== otherKey && value !== null && value !== undefined)) {
      data.push({ property: key, value: value.toString() })
    }

    if (metadata.value[otherKey]) {
      for (const [key, value] of Object.entries(metadata.value[otherKey] as Record<string, string>)) {
        data.push({ property: key, value: value.toString() })
      }
    }
  }

  return data
})

const searchedMetadata = computed(() => {
  if (!search.value) {
    return tabularMetadata.value
  }

  const searchLower = search.value.toLowerCase()

  return tabularMetadata.value.filter(({ property }) => property.toLowerCase().includes(searchLower))
})

const progress = ref<WorkerProgress>()

const warning = ref<AlertProps | undefined>({
  color: 'info',
  icon: 'heroicons:information-circle',
  title: 'Edit support coming soon',
  description: 'Editing metadata is not yet supported. Stay tuned for updates!',
})

async function startExtraction() {
  if (file.value) {
    const arraybuffer = await file.value.arrayBuffer()
    const arr = new Uint8Array(arraybuffer)

    try {
      const startTime = performance.now()

      if (!file.value) {
        toast.add({
          title: 'Error',
          icon: 'heroicons:exclamation-circle',
          description: 'No file selected',
          color: 'error',
        })
        return
      }

      const result = await extractMetadata(arr, getFileMimeType(file.value))
      metadata.value = {
        ...result,
        errors: null,
      }

      if (result.errors) {
        const errors = Array.from(new Set(result.errors))

        errors.forEach((error) => {
          toast.add({
            title: 'Invalid metadata',
            icon: 'heroicons:exclamation-circle',
            color: 'warning',
            description: error,
          })
        })
      }

      const endTime = performance.now()

      emit('extract', { inputType: getFileMimeType(file.value), duration: endTime - startTime })

      toast.add({
        title: 'Success',
        icon: 'heroicons:check-circle',
        description: `Metadata extraction completed successfully in ${(endTime - startTime).toFixed(2)}ms`,
        color: 'success',
      })
    }
    catch (e) {
      const error = parseWorkerError(e)

      toast.add({
        title: 'Error',
        icon: 'i-heroicons-exclamation-circle',
        description: error,
        color: 'error',
        actions: [{
          leadingIcon: 'i-heroicons-arrow-path',
          label: 'Retry',
          onClick: () => startExtraction(),
        }],
      })
      progress.value = undefined
    }
  }
}

function extractMetadata(arr: Uint8Array, inputType: MimeTypes): Promise<Metadata> {
  const params: MetadataWorkerRequest = { inputFile: arr, inputType }
  return runWorker<Metadata>(MetadataWorker, params, (p) => { progress.value = p })
}

watch(file, (newFile, oldFile) => {
  progress.value = undefined
  search.value = ''

  if (newFile) {
    startExtraction()
  }

  if (!newFile && oldFile) {
    metadata.value = undefined
  }
})

onMounted(async () => {
  if (file.value) {
    await nextTick()
    startExtraction()
  }
})
</script>

<template>
  <div class="w-full">
    <InputsImage v-model="file" :accept="acceptList" :hint="hint" class="w-full">
      Choose File
    </InputsImage>

    <LazyUAlert
      v-if="warning && !metadata" variant="soft" v-bind="warning"
      :close-button="{ icon: 'i-heroicons-x-mark-20-solid', color: 'gray', variant: 'link', padded: false }"
      class="mt-6" @close="warning = undefined"
    />

    <ImageWorkerProgress :progress="progress" />

    <div v-if="metadata" class="mt-6 flex-1 divide-y divide-(--ui-border-accented) w-full border border-(--ui-border) rounded-[calc(var(--ui-radius)*2)] overflow-hidden">
      <div class="flex items-center justify-between gap-2 px-4 py-3.5 overflow-x-auto">
        <h3 class="text-lg font-semibold">
          Metadata
        </h3>
        <UInput
          v-model="search"
          class="max-w-sm min-w-[12ch]"
          placeholder="Filter properties"
          :ui="{ trailing: 'pr-0.5' }"
        >
          <template v-if="search?.length" #trailing>
            <UButton
              color="neutral"
              variant="link"
              size="sm"
              icon="heroicons:x-circle"
              aria-label="Clear input"
              @click="search = ''"
            />
          </template>
        </UInput>
      </div>

      <UTable :data="searchedMetadata" />
    </div>
  </div>
</template>
