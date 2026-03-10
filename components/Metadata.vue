<script setup lang="ts">
import type { Metadata, MetadataPresets } from '#image/wasm/pkg/refilelabs_image'
import type { MetadataWorkerRequest } from '#image/workers/metadata.d'
import type { SaveMetadataWorkerRequest } from '#image/workers/save_metadata.d'
import type { WorkerProgress } from '#image/workers/shared_types'
import { runWorker } from '#image/utils/run_worker'
import init, { deletableFields, editableFields, metadataPresets } from '#image/wasm/pkg/refilelabs_image'
import { parseWorkerError } from '#image/workers/shared_types'
import MetadataWorker from '@/workers/metadata.ts?worker'
import SaveMetadataWorker from '@/workers/save_metadata.ts?worker'

export interface MetadataExtractData {
  inputType: string
  duration: number
}

export interface SaveMetadataData {
  inputType: string
  /** Which bulk-delete presets were fully applied (e.g. ['gps', 'timestamps']). */
  appliedPresets: string[]
  /** Whether any individual field values were manually edited. */
  hasManualEdits: boolean
}

const props = withDefaults(defineProps<{
  initFile?: File
  hint?: string
}>(), {
  hint: 'Any image file (i.e. png, jpg, jpeg, gif, webp, svg etc.)',
})

const emit = defineEmits<{
  extract: [opts: MetadataExtractData]
  save: [file: File, data: SaveMetadataData]
}>()

const toast = useToast()

const file = ref<File | undefined>(props.initFile)
const metadata = ref<Metadata>()

type TabularMetadata = { property: string, value: string }[]

const search = ref('')

const tabularMetadata = computed(() => {
  const data: TabularMetadata = []
  if (!metadata.value)
    return data

  for (const [key, value] of Object.entries(metadata.value)
    .filter(([key, value]) => !['other', 'gps', 'errors'].includes(key) && value != null)) {
    data.push({ property: key, value: value.toString() })
  }
  if (metadata.value.other) {
    for (const [key, value] of Object.entries(metadata.value.other as Record<string, string>)) {
      data.push({ property: key, value })
    }
  }
  if (metadata.value.gps) {
    for (const [key, value] of Object.entries(metadata.value.gps as Record<string, string>)) {
      data.push({ property: key, value })
    }
  }
  return data
})

const searchedMetadata = computed(() => {
  if (!search.value)
    return tabularMetadata.value
  const searchLower = search.value.toLowerCase()
  return tabularMetadata.value.filter(({ property }) => property.toLowerCase().includes(searchLower))
})

const progress = ref<WorkerProgress>()
const editableFieldsSet = shallowRef<Set<string>>(new Set())
const deletableFieldsSet = shallowRef<Set<string>>(new Set())

const canEditMetadata = computed(() =>
  !!file.value && new Set(['image/jpeg', 'image/png', 'image/webp', 'image/tiff']).has(getFileMimeType(file.value)))

const presets = ref<MetadataPresets>()

const editMode = ref(false)
const edits = ref<Record<string, string>>({})
const deletions = ref<Set<string>>(new Set())
// little_exif (the write library) can't remove GPS tags individually — it doesn't
// map description strings to GPS tag IDs the way it does for regular EXIF fields.
// So GPS is always removed as a block via strip_gps_tags on the Rust side.
// We still add GPS preset names to `deletions` for consistent UI state (strikethrough,
// checkbox checked), but filter them out of `changes` and pass `stripGps` separately.
const stripGps = computed(() => presets.value?.gps.some(f => deletions.value.has(f)) ?? false)
const saving = ref(false)

// GPS fields come back in a dedicated metadata.gps map (split by Rust using Context::Gps).
// We use the actual keys from that map — not the static preset list — to identify GPS rows.
const gpsPropertyNames = computed(() =>
  new Set(Object.keys(metadata.value?.gps ?? {})))

function isGpsProperty(property: string) {
  return gpsPropertyNames.value.has(property)
}

const detectedPresets = computed(() => {
  const propertySet = new Set(tabularMetadata.value.map(r => r.property))
  return {
    timestamps: (presets.value?.timestamps ?? []).some(f => propertySet.has(f)),
    device: (presets.value?.device ?? []).some(f => propertySet.has(f)),
    author: (presets.value?.author ?? []).some(f => propertySet.has(f)),
    gps: gpsPropertyNames.value.size > 0,
  }
})

const hasActiveDeletions = computed(() => deletions.value.size > 0 || stripGps.value)

function originalValue(property: string): string {
  return tabularMetadata.value.find(row => row.property === property)?.value ?? ''
}

function enterEditMode() {
  edits.value = {}
  deletions.value = new Set()
  for (const { property, value } of tabularMetadata.value) {
    if (editableFieldsSet.value.has(property))
      edits.value[property] = value
  }
  editMode.value = true
}

function exitEditMode() {
  editMode.value = false
  edits.value = {}
  deletions.value = new Set()
}

function toggleDeletion(property: string) {
  const next = new Set(deletions.value)
  if (next.has(property))
    next.delete(property)
  else
    next.add(property)
  deletions.value = next
}

function applyPreset(preset: keyof MetadataPresets) {
  const fields = presets.value?.[preset] ?? []
  const allSelected = fields.every(f => deletions.value.has(f))
  const next = new Set(deletions.value)
  if (allSelected)
    fields.forEach(f => next.delete(f))
  else
    fields.forEach(f => next.add(f))
  deletions.value = next
}

const removeMenuItems = computed(() => [[
  {
    label: detectedPresets.value.gps ? 'GPS & Location (detected)' : 'GPS & Location',
    icon: 'heroicons:map-pin',
    type: 'checkbox' as const,
    checked: stripGps.value,
    onUpdateChecked: () => applyPreset('gps'),
    onSelect: (e: Event) => e.preventDefault(),
  },
  {
    label: detectedPresets.value.timestamps ? 'Timestamps (detected)' : 'Timestamps',
    icon: 'heroicons:clock',
    type: 'checkbox' as const,
    checked: (presets.value?.timestamps ?? []).every(f => deletions.value.has(f)),
    onUpdateChecked: () => applyPreset('timestamps'),
    onSelect: (e: Event) => e.preventDefault(),
  },
  {
    label: detectedPresets.value.device ? 'Device & Lens (detected)' : 'Device & Lens',
    icon: 'heroicons:camera',
    type: 'checkbox' as const,
    checked: (presets.value?.device ?? []).every(f => deletions.value.has(f)),
    onUpdateChecked: () => applyPreset('device'),
    onSelect: (e: Event) => e.preventDefault(),
  },
  {
    label: detectedPresets.value.author ? 'Author & Rights (detected)' : 'Author & Rights',
    icon: 'heroicons:user',
    type: 'checkbox' as const,
    checked: (presets.value?.author ?? []).every(f => deletions.value.has(f)),
    onUpdateChecked: () => applyPreset('author'),
    onSelect: (e: Event) => e.preventDefault(),
  },
]])

async function saveEdited() {
  if (!file.value)
    return
  saving.value = true
  try {
    const arraybuffer = await file.value.arrayBuffer()
    const arr = new Uint8Array(arraybuffer)
    const mimeType = getFileMimeType(file.value)

    const editedFields = Object.entries(edits.value).filter(([tag, val]) => val !== originalValue(tag))

    const changes = [
      ...editedFields.map(([tag, value]) => ({ tag, value: value || null })),
      ...[...deletions.value]
        .filter(tag => !isGpsProperty(tag))
        .map(tag => ({ tag, value: null })),
    ]

    const params: SaveMetadataWorkerRequest = {
      inputFile: arr,
      inputType: mimeType,
      changes,
      stripGps: stripGps.value || undefined,
      stripAll: false,
    }
    const result = await runWorker<SaveMetadataWorkerRequest, Uint8Array>(
      SaveMetadataWorker,
      params,
      (p) => { progress.value = p },
      60_000,
    )

    const appliedPresets = (['gps', 'timestamps', 'device', 'author'] as Array<keyof MetadataPresets>)
      .filter(preset => preset === 'gps'
        ? stripGps.value
        : (presets.value?.[preset] ?? []).every(f => deletions.value.has(f)))

    const outputFile = new File([result as Uint8Array<ArrayBuffer>], file.value.name, { type: mimeType })
    emit('save', outputFile, {
      inputType: mimeType,
      appliedPresets,
      hasManualEdits: editedFields.length > 0,
    })
    exitEditMode()

    toast.add({ title: 'Success', icon: 'heroicons:check-circle', description: 'Metadata saved.', color: 'success' })
  }
  catch (e) {
    toast.add({ title: 'Error', icon: 'i-heroicons-exclamation-circle', description: parseWorkerError(e), color: 'error' })
  }
  finally {
    saving.value = false
    progress.value = undefined
  }
}

async function stripMetadata() {
  if (!file.value)
    return
  saving.value = true
  try {
    const arraybuffer = await file.value.arrayBuffer()
    const arr = new Uint8Array(arraybuffer)
    const mimeType = getFileMimeType(file.value)

    const params: SaveMetadataWorkerRequest = { inputFile: arr, inputType: mimeType, changes: [], stripAll: true }
    const result = await runWorker<SaveMetadataWorkerRequest, Uint8Array>(
      SaveMetadataWorker,
      params,
      (p) => { progress.value = p },
      60_000,
    )

    const outputFile = new File([result as Uint8Array<ArrayBuffer>], file.value.name, { type: mimeType })
    emit('save', outputFile, { inputType: mimeType, appliedPresets: ['all'], hasManualEdits: false })
    exitEditMode()

    toast.add({ title: 'Success', icon: 'heroicons:check-circle', description: 'All metadata stripped.', color: 'success' })
  }
  catch (e) {
    toast.add({ title: 'Error', icon: 'i-heroicons-exclamation-circle', description: parseWorkerError(e), color: 'error' })
  }
  finally {
    saving.value = false
    progress.value = undefined
  }
}

function extractMetadata(arr: Uint8Array, inputType: MimeTypes): Promise<Metadata> {
  const params: MetadataWorkerRequest = { inputFile: arr, inputType }
  return runWorker<MetadataWorkerRequest, Metadata>(MetadataWorker, params, (p) => {
    progress.value = p
  })
}

async function startExtraction() {
  if (!file.value)
    return

  const arraybuffer = await file.value.arrayBuffer()
  const arr = new Uint8Array(arraybuffer)

  try {
    const startTime = performance.now()
    const result = await extractMetadata(arr, getFileMimeType(file.value))
    metadata.value = { ...result, errors: null }

    if (result.errors) {
      const errors = [...new Set(result.errors)]
      errors.forEach((error) => {
        toast.add({ title: 'Invalid metadata', icon: 'heroicons:exclamation-circle', color: 'warning', description: error })
      })
    }

    const endTime = performance.now()
    emit('extract', { inputType: getFileMimeType(file.value), duration: endTime - startTime })

    toast.add({
      title: 'Success',
      icon: 'heroicons:check-circle',
      description: `Metadata extraction completed in ${(endTime - startTime).toFixed(2)}ms`,
      color: 'success',
    })
  }
  catch (e) {
    toast.add({
      title: 'Error',
      icon: 'i-heroicons-exclamation-circle',
      description: parseWorkerError(e),
      color: 'error',
      actions: [{ leadingIcon: 'i-heroicons-arrow-path', label: 'Retry', onClick: () => startExtraction() }],
    })
    progress.value = undefined
  }
}

watch(file, (newFile, oldFile) => {
  progress.value = undefined
  search.value = ''
  exitEditMode()
  if (newFile)
    startExtraction()
  if (!newFile && oldFile)
    metadata.value = undefined
})

onMounted(async () => {
  await init()
  editableFieldsSet.value = new Set(editableFields())
  deletableFieldsSet.value = new Set(deletableFields())
  presets.value = metadataPresets()
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

    <ImageWorkerProgress :progress="progress" />

    <div v-if="metadata" class="mt-6 flex-1 divide-y divide-accented w-full border border-default rounded-[calc(var(--ui-radius)*2)] overflow-hidden">
      <div class="flex items-center justify-between gap-2 px-4 py-3.5 overflow-x-auto">
        <h3 class="text-lg font-semibold">
          Metadata
        </h3>
        <div class="flex items-center gap-2 flex-wrap">
          <UInput
            v-if="!editMode"
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
          <UButton
            v-if="!editMode && canEditMetadata"
            color="neutral"
            variant="outline"
            size="sm"
            icon="heroicons:pencil"
            :disabled="saving"
            @click="enterEditMode"
          >
            Edit
          </UButton>
          <UButton
            v-if="editMode"
            color="primary"
            variant="solid"
            size="sm"
            icon="heroicons:check"
            :loading="saving"
            @click="saveEdited"
          >
            Save
          </UButton>
          <UButton
            v-if="editMode"
            color="neutral"
            variant="outline"
            size="sm"
            :disabled="saving"
            @click="exitEditMode"
          >
            Cancel
          </UButton>
          <UDropdownMenu
            v-if="editMode"
            :items="removeMenuItems"
          >
            <UButton
              color="neutral"
              :variant="hasActiveDeletions ? 'solid' : 'outline'"
              size="sm"
              icon="heroicons:trash"
              trailing-icon="heroicons:chevron-down"
              :disabled="saving"
            >
              Remove
            </UButton>
          </UDropdownMenu>
          <UTooltip text="Remove all EXIF metadata from the file">
            <UButton
              v-if="canEditMetadata"
              color="neutral"
              variant="ghost"
              size="sm"
              icon="heroicons:trash"
              :loading="saving && !editMode"
              :disabled="saving"
              @click="stripMetadata"
            >
              Strip all
            </UButton>
          </UTooltip>
        </div>
      </div>

      <UTable :data="searchedMetadata" :ui="{ base: 'table-fixed w-full' }">
        <template #value-cell="{ row }">
          <div v-if="editMode" class="flex items-center gap-1.5">
            <UInput
              v-if="editableFieldsSet.has(row.original.property) && !deletions.has(row.original.property)"
              v-model="edits[row.original.property]"
              size="sm"
              variant="outline"
              class="flex-1 min-w-0"
            />
            <span
              v-else
              :class="deletions.has(row.original.property) ? 'text-dimmed line-through' : ''"
              class="text-sm flex-1 truncate"
            >{{ row.original.value }}</span>
            <UTooltip v-if="deletableFieldsSet.has(row.original.property) || deletions.has(row.original.property)" :text="deletions.has(row.original.property) ? 'Restore field' : 'Mark for removal'">
              <UButton
                :icon="deletions.has(row.original.property) ? 'heroicons:arrow-uturn-left' : 'heroicons:x-mark'"
                color="neutral"
                variant="ghost"
                size="xs"
                @click="toggleDeletion(row.original.property)"
              />
            </UTooltip>
          </div>
          <template v-else-if="row.original.value.length > 60">
            <UTooltip :content="{ side: 'top' }">
              <span class="truncate block cursor-help">{{ row.original.value }}</span>
              <template #content>
                <span class="break-all font-mono text-xs max-w-xs block whitespace-pre-wrap">{{ row.original.value }}</span>
              </template>
            </UTooltip>
          </template>
          <span v-else>{{ row.original.value }}</span>
        </template>
      </UTable>
    </div>
  </div>
</template>
