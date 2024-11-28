<script setup lang="ts">
const props = defineProps<{
  modelValue?: number
}>()

const x = useVModel(props)

const container = ref<HTMLElement | null>(null)

const draggable = ref<HTMLElement | null>(null)

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
      ref="draggable" class="h-full absolute bg-[var(--ui-primary)]/70 w-1 cursor-ew-resize" :style="style"
    >
      <div class="h-8 w-8 absolute left-1/2 top-1/2 transform -translate-x-1/2 -translate-y-1/2 rounded-full flex items-center justify-center bg-[var(--ui-bg-accented)]">
        <UIcon name="heroicons:chevron-double-left" class="text-[var(--ui-primary)]" />
        <UIcon name="heroicons:chevron-double-right" class="text-[var(--ui-primary)]" />
      </div>
    </div>
  </div>
</template>
