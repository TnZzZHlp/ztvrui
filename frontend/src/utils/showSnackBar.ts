import { toast } from 'vue-sonner'

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
