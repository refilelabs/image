<script setup lang="ts">
import type { NavigationMenuItem } from '@nuxt/ui'

const options = [{
  label: 'Convert',
  icon: 'heroicons:arrow-path-rounded-square',
  to: '/convert',
},
// {
//  label: 'Edit',
//  icon: 'heroicons:pencil',
//  to: '/edit',
// },
{
  label: 'Compress',
  icon: 'heroicons:arrows-pointing-in',
  to: '/compress',
}, {
  label: 'Metadata',
  icon: 'heroicons:document-text',
  to: '/metadata',
}, {
  label: 'View',
  icon: 'heroicons:eye',
  to: '/view',
}] as NavigationMenuItem[]
</script>

<template>
  <Body class="h-screen" />
  <UApp>
    <SharedHeader :options="options" />
    <UContainer as="main" class="w-full flex flex-col items-center justify-around grow">
      <NuxtPage />
    </UContainer>
    <SharedFooter />
  </UApp>
</template>

<style>
#__nuxt {
  display: flex;
  flex-direction: column;
  height: 100%;
}
</style>
