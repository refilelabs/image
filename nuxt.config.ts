import { dirname } from 'node:path'
import { fileURLToPath } from 'node:url'

export const layerName = 'image'
const currentDir = dirname(fileURLToPath(import.meta.url))

// TODO: temporary workaround since c12 doesn't seem to allow overriding extends anymore
// eslint-disable-next-line node/prefer-global/process
const isLocal = process.env.npm_lifecycle_script?.replaceAll('"', '').toLowerCase().includes('--envname local')

export default defineNuxtConfig({
  extends: [isLocal ? '../base' : 'github:refilelabs/base'],

  components: [
    { path: `${currentDir}/components`, prefix: layerName },
  ],

  alias: {
    '#image': `${currentDir}`,
  },
})
