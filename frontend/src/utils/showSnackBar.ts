import { isAxiosError } from 'axios'
import { toast } from 'vue-sonner'

export function getErrorMessage(error: unknown): string {
  if (isAxiosError(error)) {
    const data = error.response?.data as { error?: string; message?: string } | undefined
    if (data?.error) return data.error
    if (data?.message) return data.message
    return error.message
  }
  if (error instanceof Error) {
    return error.message
  }
  return String(error)
}

export function showSnackBar(
  message: string,
  type: 'info' | 'error' | 'warn' | 'success' = 'info',
) {
  switch (type) {
    case 'success':
      toast.success(message)
      break
    case 'error':
      toast.error(message)
      break
    case 'warn':
      toast.warning(message)
      break
    case 'info':
    default:
      toast.info(message)
      break
  }
}
