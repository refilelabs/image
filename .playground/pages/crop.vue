<script setup lang="ts">
import type { CropResult } from '#image/components/Cropper.vue'

const callback = ({ file }: CropResult) => startDownloadFile(file)
</script>

<template>
  <ImageCropper @crop="callback" />
</template>
