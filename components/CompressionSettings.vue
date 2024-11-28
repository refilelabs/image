<script lang="ts">
export const imageTypes = ['image/jpeg', 'image/webp'] as const

export interface CompressionSettings {
  quality: number
  type: typeof imageTypes[number]
}
</script>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  quality: number
  type: CompressionSettings['type']
}>(), {
  quality: 0.7,
  type: 'image/webp',
})

const quality = useVModel(props, 'quality')
const imageType = useVModel(props, 'type')

const imageOptions = imageTypes.map(type => ({
  label: type.split('/')[1],
  value: type,
}))
</script>

<template>
  <div>
    <UPopover
      :content="{
        align: 'end',
        side: 'bottom',
        sideOffset: 8,
      }"
    >
      <UButton label="Compression Settings" color="neutral" variant="subtle" icon="heroicons:cog-6-tooth" />
      <template #content>
        <div class="p-3 min-w-30 sm:min-w-40 md:min-w-60 space-y-4">
          <UFormField label="Image Type">
            <URadioGroup v-model="imageType" :items="imageOptions" />
          </UFormField>
          <UFormField
            label="Quality" :ui="{
              container: 'flex flex-col md:flex-row md:items-center mt-0 md:mt-1',
            }"
          >
            <span class="mb-1 md:mb-0 md:mr-2">{{ (quality * 100).toFixed(0) }}%</span>
            <USlider v-model="quality" :min="0" :max="1" :step="0.01" />
          </UFormField>
        </div>
      </template>
    </UPopover>
  </div>
</template>
