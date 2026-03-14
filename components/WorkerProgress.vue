<script setup lang="ts">
import type { WorkerProgress } from '#image/workers/shared_types'

defineProps<{ progress: WorkerProgress | undefined }>()
</script>

<template>
  <div v-if="progress !== undefined" class="mt-4 rounded-xl border border-default bg-elevated p-4">
    <div class="flex items-center justify-between mb-2">
      <span class="text-sm text-muted">{{ progress.message }}</span>
      <span class="text-xs tabular-nums text-dimmed">{{ progress.progress }}%</span>
    </div>
    <UProgress :model-value="progress.progress" />
  </div>
</template>
