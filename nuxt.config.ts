// import { layerList, getOtherLayers } from '../layerList'
import { dirname } from 'node:path'
import { fileURLToPath } from 'node:url'

export const layerName = 'image'
const currentDir = dirname(fileURLToPath(import.meta.url))

export default defineNuxtConfig({
  // extends: [
  //  'github:refilelabs/base',
  // ],

  extends: ['../base'],

  components: [
    `${currentDir}/components`,
    // ...layerList.filter((layer) => layer !== layerName).map((layer) => ({ path: `../${layer}/components`, pathPrefix: true })),
    // getOtherLayers(layerName).map((layer) => ({ path: `../${layer}/components`, pathPrefix: true })),
  ],

  alias: {
    '#image': `${currentDir}`,
  },

  // $env: {
  //  $local: {
  //    extends: ['../base'],
  //  },
  // },
  //
  // $production: {
  //  extends: ['../base'],
  // },
})
