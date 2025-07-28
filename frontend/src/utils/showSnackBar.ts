import { createApp } from 'vue'
import snackBar from '../components/snackBar.vue'

let currentSnackBar: HTMLDivElement | null = null
let timer: ReturnType<typeof setTimeout> | null = null

export function showSnackBar(
  message: string,
  type: 'info' | 'error' | 'warn' | 'success' = 'info',
) {
  if (currentSnackBar) {
    currentSnackBar.remove()
  }

  if (timer) {
    clearTimeout(timer)
  }

  const snackBarNode = document.createElement('div')
  document.body.appendChild(snackBarNode)
  const snackBarInstance = createApp(snackBar, { message, type })
  snackBarInstance.mount(snackBarNode)

  currentSnackBar = snackBarNode

  timer = setTimeout(() => {
    document.body.removeChild(snackBarNode)
    currentSnackBar = null
  }, 3000) // Automatically remove after 3 seconds
}
