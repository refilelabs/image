<script setup lang="ts">
import type { ResizeResult } from '#image/components/Resizer.vue'

const callback = ({ file }: ResizeResult) => startDownloadFile(file)
</script>

<template>
  <ImageResizer @resize="callback" />
</template>
