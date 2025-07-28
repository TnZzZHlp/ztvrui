import type { PeerInfo } from '@/types/zerotier/peers'

export function listPeers(): Promise<PeerInfo[]> {
  return fetch(`/ztapi/peer`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error fetching node status: ${response.statusText}`)
    }
    return response.json()
  })
}

export function getJoinedNetworkById(memberId: string): Promise<PeerInfo> {
  return fetch(`/ztapi/peer/${memberId}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error fetching joined network by Id: ${response.statusText}`)
    }
    return response.json()
  })
}
