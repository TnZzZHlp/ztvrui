import i18n from '@/i18n'
import { showSnackBar } from './showSnackBar'

export function copyToClipboard(text: string) {
  if (navigator.clipboard?.writeText) {
    return navigator.clipboard
      .writeText(text)
      .then(() => {
        showSnackBar(i18n.global.t('common.copied'), 'success')
      })
      .catch(() => {
        fallbackCopy(text)
      })
  }
  fallbackCopy(text)
}

function fallbackCopy(text: string) {
  const textArea = document.createElement('textarea')
  textArea.value = text
  textArea.setAttribute('readonly', '')
  textArea.style.position = 'fixed'
  textArea.style.opacity = '0'
  textArea.style.pointerEvents = 'none'
  document.body.appendChild(textArea)
  textArea.select()
  try {
    const ok = document.execCommand('copy')
    showSnackBar(
      i18n.global.t(ok ? 'common.copied' : 'common.copyFailed'),
      ok ? 'success' : 'error',
    )
  } catch {
    showSnackBar(i18n.global.t('common.copyFailed'), 'error')
  } finally {
    document.body.removeChild(textArea)
  }
}
