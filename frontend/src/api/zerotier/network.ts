import type { Result } from '@/types/zerotier/common'
import type { LocalNetworkInfo, LocalNetworkSettings } from '@/types/zerotier/network'

export function joinOrUpdateNetworkSettings(
  networkId: string,
  settings: LocalNetworkSettings,
): Promise<LocalNetworkInfo> {
  return fetch(`/ztapi/network/${networkId}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(settings),
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error updating network settings: ${response.statusText}`)
    }
    return response.json()
  })
}

export function listNetworks(): Promise<LocalNetworkInfo[]> {
  return fetch(`/ztapi/network`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error listing networks: ${response.statusText}`)
    }
    return response.json()
  })
}

export function leaveNetwork(networkId: string): Promise<Result> {
  return fetch(`/ztapi/network/${networkId}`, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
    },
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error leaving network: ${response.statusText}`)
    }
    return response.json()
  })
}

export function getJoinedNetworkById(networkId: string): Promise<LocalNetworkInfo> {
  return fetch(`/ztapi/network/${networkId}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error fetching network: ${response.statusText}`)
    }
    return response.json()
  })
}
