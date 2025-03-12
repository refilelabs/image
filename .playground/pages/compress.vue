<script setup lang="ts">
import type { CompressionResult } from '#image/components/Compressor.vue'

const callback = ({ file }: CompressionResult) => startDownloadFile(file)
</script>

<template>
  <ImageCompressor @compress="callback" />
</template>
