// import { layerList, getOtherLayers } from '../layerList'
import { dirname } from 'node:path'
import { fileURLToPath } from 'node:url'

export const layerName = 'image'
const currentDir = dirname(fileURLToPath(import.meta.url))

export default defineNuxtConfig({
  extends: [
   'github:refilelabs/base',
  ],

  components: [
    { path: `${currentDir}/components`, prefix: layerName },
  ],

  alias: {
    '#image': `${currentDir}`,
  },

  $env: {
    $local: {
      extends: ['../base'],
    },
  }
})
