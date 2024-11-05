import antfu from '@antfu/eslint-config'
import withNuxt from './.playground/.nuxt/eslint.config.mjs'

export default withNuxt(
  antfu({
    typescript: {
      tsconfigPath: './tsconfig.json',
    },
    rules: {
      'antfu/top-level-function': 'off',
    },
  }),
  {
    ignores: ['wasm/*'],
  },
)
