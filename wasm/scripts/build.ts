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
  license: 'MIT',
  homepage: 'https://refilelabs.com/image',
  keywords: ['image', 'processing', 'wasm', 'web'],
  repository: {
    type: 'git',
    url: 'git+https://github.com/refilelabs/image.git',
  },
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

const wasm_pack_extras = ['package.json', 'LICENSE', 'README.md', '.gitignore']
await Promise.all(
  ['bundler', 'node', 'web'].flatMap(dir =>
    wasm_pack_extras.map(file => $`rm -f ${wasmDir}/pkg/${dir}/${file}`),
  ),
)

for (const dir of ['', 'bundler', 'node', 'web']) {
  await Bun.write(`${wasmDir}/pkg/${dir ? `${dir}/` : ''}.npmignore`, '')
}
