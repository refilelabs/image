<script setup lang="ts">
import type { WorkerProgress } from '#image/workers/shared_types'

defineProps<{ progress: WorkerProgress | undefined }>()
</script>

<template>
  <aside v-if="progress !== undefined" class="pt-6 w-full text-center">
    <UProgress :model-value="progress.progress" />
    {{ progress.message }}
  </aside>
</template>
