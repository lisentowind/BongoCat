import type { Monitor, PhysicalPosition } from '@tauri-apps/api/window'

import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { cursorPosition, monitorFromPoint } from '@tauri-apps/api/window'

// 缓存显示器信息 减少重复的异步调用
let cachedMonitor: Monitor | null = null
let cachedScaleFactor: number | null = null
let cacheTimestamp = 0
const CACHE_DURATION = 1000

export async function getCursorMonitor(cursorPoint?: PhysicalPosition) {
  cursorPoint ??= await cursorPosition()

  const now = Date.now()
  const appWindow = getCurrentWebviewWindow()

  // 缓存有效 直接使用
  if (now - cacheTimestamp < CACHE_DURATION && cachedScaleFactor !== null) {
    const { x, y } = cursorPoint.toLogical(cachedScaleFactor)
    const monitor = await monitorFromPoint(x, y)

    // 鼠标还在同一个显示器上 直接返回缓存的 monitor
    if (monitor && cachedMonitor && monitor.name === cachedMonitor.name) {
      return cachedMonitor
    }

    // 鼠标移动到了不同的显示器 更新缓存
    if (monitor) {
      cachedMonitor = monitor
      cacheTimestamp = now
      return monitor
    }
  }

  // 缓存过期或首次调用 重新获取
  const scaleFactor = await appWindow.scaleFactor()
  cachedScaleFactor = scaleFactor

  const { x, y } = cursorPoint.toLogical(scaleFactor)
  const monitor = await monitorFromPoint(x, y)

  if (!monitor) return

  cachedMonitor = monitor
  cacheTimestamp = now

  return monitor
}
