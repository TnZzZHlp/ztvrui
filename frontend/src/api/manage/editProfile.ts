import type { Auth } from '@/types/manage'
import apiClient from '@/utils/axios'

export async function editProfile(auth: Auth): Promise<void> {
  await apiClient.post('/api/editprofile', auth)
}
