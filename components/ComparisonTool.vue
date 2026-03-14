<script setup lang="ts">
const props = defineProps<{
  modelValue?: number
}>()

const x = useVModel(props)

const container = useTemplateRef('container')

const draggable = useTemplateRef('draggable')

const style = ref<string>('')

onMounted(async () => {
  if (container.value && draggable.value) {
    const { left, right } = useElementBounding(container)

    const validLeftRight = computed(() => left.value !== 0 || right.value !== 0)

    await until(validLeftRight).toBe(true)

    const { style: styleVar, x: xVar } = useDraggable(draggable, {
      initialValue: {
        x: (right.value - left.value) / 2,
        y: 0,
      },
      axis: 'x',
      containerElement: container,
      stopPropagation: true,
      capture: true,
    })

    const correctedX = computed(() => xVar.value + left.value)

    x.value = correctedX.value
    watch(correctedX, (value) => {
      x.value = value
    })

    style.value = styleVar.value
    watch(styleVar, (value) => {
      style.value = value
    })
  }
})
</script>

<template>
  <div ref="container" class="h-full w-full relative">
    <div class="h-full w-full">
      <slot />
    </div>
    <div
      ref="draggable"
      class="h-full absolute w-px bg-white/80 shadow-[0_0_8px_rgba(0,0,0,0.4)] cursor-ew-resize"
      :style="style"
    >
      <!-- Handle -->
      <div class="h-9 w-9 absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rounded-full flex items-center justify-center bg-white shadow-lg">
        <UIcon name="heroicons:arrows-right-left" class="w-4 h-4 text-gray-700" />
      </div>
      <!-- Before / After labels -->
      <span class="absolute top-1/2 -translate-y-1/2 right-full mr-6 px-2 py-0.5 rounded-md bg-black/40 backdrop-blur-sm text-white text-xs font-medium pointer-events-none select-none whitespace-nowrap">Before</span>
      <span class="absolute top-1/2 -translate-y-1/2 left-full ml-6 px-2 py-0.5 rounded-md bg-black/40 backdrop-blur-sm text-white text-xs font-medium pointer-events-none select-none whitespace-nowrap">After</span>
    </div>
  </div>
</template>
