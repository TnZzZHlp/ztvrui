import type { NodeStatus } from '@/types/zerotier/nodeStatus'

export function nodeStatus(): Promise<NodeStatus> {
  return fetch(`/ztapi/status`, {
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
