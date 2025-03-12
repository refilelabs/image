<script setup lang="ts">
import type { ConversionResult } from '#image/components/Converter.vue'

const callback = ({ file }: ConversionResult) => startDownloadFile(file)
</script>

<template>
  <ImageConverter @convert="callback" />
</template>
