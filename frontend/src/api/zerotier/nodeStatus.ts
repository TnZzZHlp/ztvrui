import type { NodeStatus } from '@/types/zerotier/nodeStatus'
import apiClient from '@/utils/axios'

export async function nodeStatus(): Promise<NodeStatus> {
  const { data } = await apiClient.get<NodeStatus>('/ztapi/status')
  return data
}
