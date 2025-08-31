import type { Result } from '@/types/zerotier/common'
import type { LocalNetworkInfo, LocalNetworkSettings } from '@/types/zerotier/network'
import { authenticatedFetch } from '@/utils/apiUtils'

export async function joinOrUpdateNetworkSettings(
  networkId: string,
  settings: LocalNetworkSettings,
): Promise<LocalNetworkInfo> {
  const response = await authenticatedFetch(`/ztapi/network/${networkId}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(settings),
  })

  if (!response.ok) {
    throw new Error(`Error updating network settings: ${response.statusText}`)
  }
  return response.json()
}

export async function listNetworks(): Promise<LocalNetworkInfo[]> {
  const response = await authenticatedFetch(`/ztapi/network`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  })

  if (!response.ok) {
    throw new Error(`Error listing networks: ${response.statusText}`)
  }
  return response.json()
}

export async function leaveNetwork(networkId: string): Promise<Result> {
  const response = await authenticatedFetch(`/ztapi/network/${networkId}`, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
    },
  })

  if (!response.ok) {
    throw new Error(`Error leaving network: ${response.statusText}`)
  }
  return response.json()
}

export async function getJoinedNetworkById(networkId: string): Promise<LocalNetworkInfo> {
  const response = await authenticatedFetch(`/ztapi/network/${networkId}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  })

  if (!response.ok) {
    throw new Error(`Error fetching network: ${response.statusText}`)
  }
  return response.json()
}
