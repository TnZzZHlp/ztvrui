import type {
  ControllerNetworkInfo,
  ControllerStatusInfo,
  Members,
  ControllerNetworkMemberInfo,
  ControllerNetworkMemberSettings,
} from '@/types/zerotier/controller'
import apiClient from '@/utils/axios'

export async function controllerStatus(): Promise<ControllerStatusInfo> {
  const { data } = await apiClient.get<ControllerStatusInfo>('/ztapi/controller')
  return data
}

export async function listNetworkIds(): Promise<string[]> {
  const { data } = await apiClient.get<string[]>('/ztapi/controller/network')
  return data
}

export async function generateRandomNetworkId(
  networkInfo: ControllerNetworkInfo,
): Promise<ControllerNetworkInfo> {
  const { data } = await apiClient.post<ControllerNetworkInfo>(
    '/ztapi/controller/network',
    networkInfo,
  )
  return data
}

export async function getNetworkById(networkId: string): Promise<ControllerNetworkInfo> {
  const { data } = await apiClient.get<ControllerNetworkInfo>(
    `/ztapi/controller/network/${networkId}`,
  )
  return data
}

export async function createOrUpdateNetwork(
  networkId: string,
  controllerinfo: ControllerNetworkInfo,
): Promise<ControllerNetworkInfo> {
  const { data } = await apiClient.post<ControllerNetworkInfo>(
    `/ztapi/controller/network/${networkId}`,
    controllerinfo,
  )
  return data
}

export async function deleteNetwork(networkId: string): Promise<void> {
  await apiClient.delete(`/ztapi/controller/network/${networkId}`)
}

export async function listNetworkMemberIds(networkId: string): Promise<Members> {
  const { data } = await apiClient.get<Members>(`/ztapi/controller/network/${networkId}/member`)
  return data
}

export async function getNetworkMemberById(
  networkId: string,
  memberId: string,
): Promise<ControllerNetworkMemberInfo> {
  const { data } = await apiClient.get<ControllerNetworkMemberInfo>(
    `/ztapi/controller/network/${networkId}/member/${memberId}`,
  )
  return data
}

export async function createOrUpdateNetworkMember(
  networkId: string,
  memberId: string,
  memberSettings: ControllerNetworkMemberSettings,
): Promise<ControllerNetworkMemberInfo> {
  const { data } = await apiClient.post<ControllerNetworkMemberInfo>(
    `/ztapi/controller/network/${networkId}/member/${memberId}`,
    memberSettings,
  )
  return data
}

export async function deleteNetworkMember(
  networkId: string,
  memberId: string,
): Promise<ControllerNetworkMemberInfo> {
  const { data } = await apiClient.delete<ControllerNetworkMemberInfo>(
    `/ztapi/controller/network/${networkId}/member/${memberId}`,
  )
  return data
}
