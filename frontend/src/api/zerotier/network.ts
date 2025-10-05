import type { Result } from '@/types/zerotier/common'
import type { LocalNetworkInfo, LocalNetworkSettings } from '@/types/zerotier/network'
import apiClient from '@/utils/axios'

export async function joinOrUpdateNetworkSettings(
  networkId: string,
  settings: LocalNetworkSettings,
): Promise<LocalNetworkInfo> {
  const { data } = await apiClient.post<LocalNetworkInfo>(`/ztapi/network/${networkId}`, settings)
  return data
}

export async function listNetworks(): Promise<LocalNetworkInfo[]> {
  const { data } = await apiClient.get<LocalNetworkInfo[]>('/ztapi/network')
  return data
}

export async function leaveNetwork(networkId: string): Promise<Result> {
  const { data } = await apiClient.delete<Result>(`/ztapi/network/${networkId}`)
  return data
}

export async function getJoinedNetworkById(networkId: string): Promise<LocalNetworkInfo> {
  const { data } = await apiClient.get<LocalNetworkInfo>(`/ztapi/network/${networkId}`)
  return data
}
