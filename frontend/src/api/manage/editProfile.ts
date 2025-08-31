import type { Auth } from '@/types/manage'
import { authenticatedFetch } from '@/utils/apiUtils'

export function editProfile(auth: Auth): Promise<void> {
  return authenticatedFetch(`/api/editprofile`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(auth),
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error editing profile: ${response.statusText}`)
    }
  })
}
