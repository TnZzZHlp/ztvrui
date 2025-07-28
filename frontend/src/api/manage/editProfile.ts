import type { Auth } from '@/types/manage'

export function editProfile(auth: Auth): Promise<void> {
  return fetch(`/api/editprofile`, {
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
