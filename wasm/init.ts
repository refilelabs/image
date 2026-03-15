import init from './pkg/web/refilelabs_image'

let _promise: Promise<void> | null = null

export function ensureInit(): Promise<void> {
  if (!_promise) {
    _promise = init().then(() => {}).catch((err) => { _promise = null; throw err })
  }
  return _promise
}
