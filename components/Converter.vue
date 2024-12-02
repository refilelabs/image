<script setup lang="ts">
import type { SVGData } from '#image/utils/dimensions'
import type { SvgSettings } from '#image/wasm/pkg/image'
// https://github.com/eliaSchenker/nuxt-webworker/blob/main/plugins/sw.ts
import type { ConvertWorkerMessage, ConvertWorkerRequest } from '#image/workers/convert.d'
import type { AlertProps } from '@nuxt/ui'
import { acceptList, getEndingMimeType } from '#image/utils/file_types'
import { WorkerMessageType, type WorkerProgress } from '#image/workers/shared_types'
import ConversionWorker from '@/workers/convert.ts?worker'

export interface ConversionData {
  inputType: string
  outputType: string
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
  convert: [opts: ConversionData]
}>()

const toast = useToast()

const file = ref<File | undefined>(props.initFile)
const svgData = ref<SVGData>()

const size = ref<[number, number]>()

const progress = ref<WorkerProgress>()

const outputType = ref(props.initOutputType ? getEndingMimeType(props.initOutputType) ?? 'image/jpeg' : 'image/jpeg')

const warning = ref<AlertProps>()

// display warnings depending on input/output types
watchEffect(() => {
  if (!file.value) {
    return
  }

  switch (outputType.value) {
    case 'image/x-icon':
      warning.value = {
        title: 'Warning',
        icon: 'heroicons-exclamation-circle',
        description: 'Converting an image to an icon file will resize it to 256x256',
      }
      break
    default:
      warning.value = undefined
  }

  switch (file.value.type) {
    case 'image/svg+xml':
      warning.value = {
        title: 'Warning',
        icon: 'heroicons-exclamation-circle',
        description: 'Text elements may be lost when converting SVG files, make sure to check the output',
      }
      break
    default:
      warning.value = undefined
  }
})

// Read SVG data from file
watchEffect(() => {
  if (file.value && file.value.type === 'image/svg+xml') {
    const reader = new FileReader()
    reader.onloadend = (e) => {
      const res = e.target?.result as string
      const parser = new DOMParser()
      const doc = parser.parseFromString(res, 'image/svg+xml')
      const svg = doc.querySelector('svg')
      if (svg) {
        const viewBox = svg.getAttribute('viewBox')
        if (viewBox) {
          // aspectRatioModel.value = Dimensions.fromSVGViewBox(viewBox)
          const dimensions = Dimensions.fromSVGViewBox(viewBox)
          svgData.value = dimensions.getSVGData()
        }
      }
    }
    reader.readAsText(file.value)
  }
  else {
    svgData.value = undefined
  }
})

// Get SVG dimensions
watchEffect(() => {
  if (svgData.value) {
    size.value = [svgData.value.width, svgData.value.height]
  }
  else {
    size.value = undefined
  }
})

async function startConversion() {
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

      const result = await convert(arr, getFileMimeType(file.value), outputType.value as keyof typeof outputFileEndings)

      if (result && result.length)
        startDownload(result, `converted.${inputFileEndings[outputType.value as keyof typeof outputFileEndings]}`)

      const endTime = performance.now()

      emit('convert', { inputType: file.value.type, outputType: outputType.value, duration: endTime - startTime })

      toast.add({
        title: 'Success',
        icon: 'heroicons:check-circle',
        description: `Conversion completed successfully in ${(endTime - startTime).toFixed(2)}ms`,
        color: 'success',
      })
    }
    catch (e) {
      let error = (e as any).message || (e as any).toString()

      error = error.replace(/^Error: /, '')

      toast.add({
        title: 'Error',
        icon: 'i-heroicons-exclamation-circle',
        description: error,
        color: 'error',
        actions: [{
          leadingIcon: 'i-heroicons-arrow-path',
          label: 'Retry',
          onClick: () => startConversion(),
        }],
      })

      progress.value = undefined
    }
  }
}

function convert(arr: Uint8Array, inputType: MimeTypes, outputType: keyof typeof outputFileEndings): Promise<Uint8Array> {
  // eslint-disable-next-line no-async-promise-executor
  return new Promise(async (resolve, reject) => {
    const params: ConvertWorkerRequest = {
      inputFile: arr,
      inputType,
      outputType,
    }

    if (svgData.value && size.value) {
      Object.assign(params, {
        settings: {
          type: 'svg',
          width: size.value[0],
          height: size.value[1],
        } as SvgSettings,
      })
    }

    const { data, post, terminate } = useWebWorker(new ConversionWorker())

    post(params)

    while (true) {
      try {
        await until(data).changed({ timeout: 1000 })
      }
      catch {
        reject(new Error('Conversion timed out'))
      }

      const res = data as Ref<ConvertWorkerMessage>

      switch (res.value.type) {
        case WorkerMessageType.DONE:
          resolve(res.value.payload.data)
          break
        case WorkerMessageType.ERROR:
          reject(res.value.payload.error)
          break
        case WorkerMessageType.PROGRESS:
          progress.value = res.value.payload
          break
      }

      if (res.value.type === WorkerMessageType.DONE || res.value.type === WorkerMessageType.ERROR) {
        terminate()
        break
      }
    }
  })
}

watch(file, () => {
  progress.value = undefined
})
</script>

<template>
  <div>
    <InputsImage v-model="file" :hint="hint" :accept="acceptList" class="w-full">
      Choose File
    </InputsImage>

    <div class="flex flex-row items-end space-x-10 pt-3">
      <div class="grow">
        <UFormField label="Output Type">
          <USelect v-model="outputType" :items="Object.entries(outputFileEndings).map(([ending, imageType]) => ({ value: ending, label: imageType }))" value-key="value" label-key="label" />
        </UFormField>
      </div>
      <UButton class="cursor-pointer" @click="startConversion">
        Start Conversion
      </UButton>
    </div>

    <LazyUAlert
      v-if="warning" variant="soft" color="warning" v-bind="warning"
      :close-button="{ icon: 'i-heroicons-x-mark-20-solid', color: 'gray', variant: 'link', padded: false }"
      class="mt-6" @close="warning = undefined"
    />

    <UCard v-if="svgData !== undefined && size !== undefined" class="mt-6">
      <template #header>
        Svg Settings
      </template>
      <LazyInputsSize v-model:size="size" :aspect-ratio="svgData ? svgData.aspectRatio : 2" :source-dimensions="[svgData ? svgData.width : 0, svgData ? svgData.height : 0]" />
    </UCard>

    <aside v-if="progress !== undefined" class="pt-6 w-full text-center">
      <UProgress v-model="progress.progress" />
      {{ progress.message }}
    </aside>
  </div>
</template>
