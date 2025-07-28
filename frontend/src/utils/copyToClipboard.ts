import i18n from '@/i18n'
import { showSnackBar } from './showSnackBar'

export function copyToClipboard(text: string) {
  // Determine whether the Clipboard API is supported
  if (navigator.clipboard) {
    return navigator.clipboard
      .writeText(text)
      .then(() => {
        showSnackBar(i18n.global.t('common.copied'), 'success')
      })
      .catch(() => {
        showSnackBar(i18n.global.t('common.copyFailed'), 'error')
      })
  } else {
    // Fallback handling
    const textArea = document.createElement('textarea')
    textArea.value = text
    document.body.appendChild(textArea)
    textArea.select()
    document.execCommand('copy')
    document.body.removeChild(textArea)
    showSnackBar(i18n.global.t('common.copied'), 'success')
  }
}
