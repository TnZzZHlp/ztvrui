import type { PeerInfo } from '@/types/zerotier/peers'
import apiClient from '@/utils/axios'

export async function listPeers(): Promise<PeerInfo[]> {
  const { data } = await apiClient.get<PeerInfo[]>('/ztapi/peer')
  return data
}

export async function getJoinedNetworkById(memberId: string): Promise<PeerInfo> {
  const { data } = await apiClient.get<PeerInfo>(`/ztapi/peer/${memberId}`)
  return data
}
