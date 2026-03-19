import type { Ref } from 'vue'

export interface CropRect {
  x: number
  y: number
  w: number
  h: number
}

export type DragMode = 'move' | 'draw' | 'NW' | 'NE' | 'SW' | 'SE' | 'N' | 'S' | 'E' | 'W' | null

export const HANDLE_RADIUS = 12
export const HANDLE_SIZE = 14

export function getHandles(c: CropRect) {
  const { x, y, w, h } = c
  return {
    NW: { x, y },
    N: { x: x + w / 2, y },
    NE: { x: x + w, y },
    E: { x: x + w, y: y + h / 2 },
    SE: { x: x + w, y: y + h },
    S: { x: x + w / 2, y: y + h },
    SW: { x, y: y + h },
    W: { x, y: y + h / 2 },
  }
}

function clamp(val: number, min: number, max: number) {
  return Math.max(min, Math.min(max, val))
}

const aspectRatioMap: Record<string, number> = {
  '1:1': 1,
  '16:9': 16 / 9,
  '4:3': 4 / 3,
  '3:2': 3 / 2,
}

interface UseCropDragOptions {
  crop: Ref<CropRect | undefined>
  originalSize: Ref<[number, number]>
  displayScale: Ref<{ x: number, y: number }>
  displayCanvas: Ref<HTMLCanvasElement | null>
  aspectLock: Ref<string>
}

export function useCropDrag({ crop, originalSize, displayScale, displayCanvas, aspectLock }: UseCropDragOptions) {
  const dragMode = ref<DragMode>(null)
  const dragStart = ref({ x: 0, y: 0 })
  const dragStartCrop = ref<CropRect>({ x: 0, y: 0, w: 0, h: 0 })

  function toImageCoords(clientX: number, clientY: number) {
    const canvas = displayCanvas.value!
    const rect = canvas.getBoundingClientRect()
    const cssScaleX = canvas.width / rect.width
    const cssScaleY = canvas.height / rect.height
    return {
      x: (clientX - rect.left) * cssScaleX / displayScale.value.x,
      y: (clientY - rect.top) * cssScaleY / displayScale.value.y,
    }
  }

  function getCanvasCoords(clientX: number, clientY: number) {
    const canvas = displayCanvas.value!
    const rect = canvas.getBoundingClientRect()
    const cssScaleX = canvas.width / rect.width
    const cssScaleY = canvas.height / rect.height
    return {
      x: (clientX - rect.left) * cssScaleX,
      y: (clientY - rect.top) * cssScaleY,
    }
  }

  function hitTestHandle(canvasX: number, canvasY: number): DragMode {
    if (!crop.value)
      return null
    const { x, y, w, h } = crop.value
    const { x: sx, y: sy } = displayScale.value
    const handles = getHandles({ x: x * sx, y: y * sy, w: w * sx, h: h * sy })
    for (const [key, pt] of Object.entries(handles)) {
      if (Math.abs(canvasX - pt.x) <= HANDLE_RADIUS && Math.abs(canvasY - pt.y) <= HANDLE_RADIUS)
        return key as DragMode
    }
    return null
  }

  function isInsideCrop(canvasX: number, canvasY: number): boolean {
    if (!crop.value)
      return false
    const { x, y, w, h } = crop.value
    const { x: sx, y: sy } = displayScale.value
    return canvasX >= x * sx && canvasX <= (x + w) * sx
      && canvasY >= y * sy && canvasY <= (y + h) * sy
  }

  function applyAspectLock(rect: CropRect, anchor: string): CropRect {
    if (aspectLock.value === 'free')
      return rect
    const ratio = aspectRatioMap[aspectLock.value]
    if (ratio === undefined)
      return rect
    const newH = Math.round(rect.w / ratio)
    if (anchor.includes('S') || !anchor.includes('N'))
      return { ...rect, h: newH }
    return { ...rect, y: rect.y + rect.h - newH, h: newH }
  }

  function onMouseDown(e: MouseEvent) {
    if (!crop.value || !displayCanvas.value)
      return
    const { x: cx, y: cy } = getCanvasCoords(e.clientX, e.clientY)
    const handle = hitTestHandle(cx, cy)
    if (handle)
      dragMode.value = handle
    else if (isInsideCrop(cx, cy))
      dragMode.value = 'move'
    else
      dragMode.value = 'draw'
    dragStart.value = toImageCoords(e.clientX, e.clientY)
    dragStartCrop.value = { ...crop.value }
  }

  function onMouseMove(e: MouseEvent) {
    if (!dragMode.value || !crop.value || !displayCanvas.value)
      return

    const imgCoords = toImageCoords(e.clientX, e.clientY)
    const dx = imgCoords.x - dragStart.value.x
    const dy = imgCoords.y - dragStart.value.y
    const [imgW, imgH] = originalSize.value
    const start = dragStartCrop.value

    let next = { ...crop.value }

    if (dragMode.value === 'draw') {
      const x0 = clamp(Math.round(dragStart.value.x), 0, imgW)
      const y0 = clamp(Math.round(dragStart.value.y), 0, imgH)
      const x1 = clamp(Math.round(imgCoords.x), 0, imgW)
      const y1 = clamp(Math.round(imgCoords.y), 0, imgH)
      next = { x: Math.min(x0, x1), y: Math.min(y0, y1), w: Math.abs(x1 - x0), h: Math.abs(y1 - y0) }
      if (aspectLock.value !== 'free')
        next = applyAspectLock(next, 'SE')
    }
    else if (dragMode.value === 'move') {
      next.x = Math.round(clamp(start.x + dx, 0, imgW - start.w))
      next.y = Math.round(clamp(start.y + dy, 0, imgH - start.h))
    }
    else {
      const mode = dragMode.value
      if (mode === 'NW' || mode === 'W' || mode === 'SW') {
        const newX = Math.round(clamp(start.x + dx, 0, start.x + start.w - 1))
        next.w = start.x + start.w - newX
        next.x = newX
      }
      if (mode === 'NE' || mode === 'E' || mode === 'SE')
        next.w = Math.round(clamp(start.w + dx, 1, imgW - start.x))
      if (mode === 'NW' || mode === 'N' || mode === 'NE') {
        const newY = Math.round(clamp(start.y + dy, 0, start.y + start.h - 1))
        next.h = start.y + start.h - newY
        next.y = newY
      }
      if (mode === 'SW' || mode === 'S' || mode === 'SE')
        next.h = Math.round(clamp(start.h + dy, 1, imgH - start.y))
      if (aspectLock.value !== 'free')
        next = applyAspectLock(next, mode)
    }

    if (next.w >= 1 && next.h >= 1)
      crop.value = next
  }

  function onMouseUp() {
    dragMode.value = null
  }

  function onTouchStart(e: TouchEvent) {
    if (e.touches.length === 1)
      onMouseDown(e.touches[0] as unknown as MouseEvent)
  }

  function onTouchMove(e: TouchEvent) {
    if (e.touches.length === 1) {
      e.preventDefault()
      onMouseMove(e.touches[0] as unknown as MouseEvent)
    }
  }

  function selectAll() {
    const [w, h] = originalSize.value
    if (w && h)
      crop.value = { x: 0, y: 0, w, h }
  }

  function onNumericInput(field: keyof CropRect, val: string) {
    if (!crop.value)
      return
    const num = Number.parseInt(val, 10)
    if (Number.isNaN(num))
      return
    const [imgW, imgH] = originalSize.value
    const c = { ...crop.value }
    if (field === 'x')
      c.x = clamp(num, 0, imgW - c.w)
    else if (field === 'y')
      c.y = clamp(num, 0, imgH - c.h)
    else if (field === 'w')
      c.w = clamp(num, 1, imgW - c.x)
    else if (field === 'h')
      c.h = clamp(num, 1, imgH - c.y)
    crop.value = c
  }

  useEventListener(window, 'mousemove', onMouseMove)
  useEventListener(window, 'mouseup', onMouseUp)

  return { onMouseDown, onTouchStart, onTouchMove, onTouchEnd: onMouseUp, selectAll, onNumericInput }
}
