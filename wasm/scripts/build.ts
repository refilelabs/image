import { $ } from 'bun'
import { readPackageJSON, writePackageJSON } from 'pkg-types'

const wasmDir = import.meta.dir + '/..'

await Promise.all([
  $`wasm-pack build --target bundler --out-dir pkg/bundler --release --features wasm`.cwd(wasmDir),
  $`wasm-pack build --target nodejs  --out-dir pkg/node    --release --features wasm`.cwd(wasmDir),
  $`wasm-pack build --target web     --out-dir pkg/web     --release --features wasm`.cwd(wasmDir),
])

const base = await readPackageJSON(`${wasmDir}/pkg/bundler/package.json`)

await writePackageJSON(`${wasmDir}/pkg/package.json`, {
  name: '@refilelabs/image',
  version: base.version,
  description: base.description,
  license: base.license,
  repository: base.repository,
  exports: {
    '.': {
      node: { types: './node/refilelabs_image.d.ts', default: './node/refilelabs_image.js' },
      browser: { types: './web/refilelabs_image.d.ts', default: './web/refilelabs_image.js' },
      default: { types: './bundler/refilelabs_image.d.ts', default: './bundler/refilelabs_image.js' },
    },
  },
  files: ['bundler/', 'node/', 'web/'],
  sideEffects: false,
})

await Bun.write(`${wasmDir}/pkg/README.md`, Bun.file(`${wasmDir}/README_JS.md`))
await Bun.write(`${wasmDir}/pkg/.npmignore`, '')
