import type {
  ControllerNetworkInfo,
  ControllerStatusInfo,
  Members,
  ControllerNetworkMemberInfo,
  ControllerNetworkMemberSettings,
} from '@/types/zerotier/controller'

export function controllerStatus(): Promise<ControllerStatusInfo> {
  return fetch(`/ztapi/controller`, {
    method: 'GET',
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error fetching controller status: ${response.statusText}`)
    }
    return response.json()
  })
}

export function listNetworkIds(): Promise<string[]> {
  return fetch(`/ztapi/controller/network`, {
    method: 'GET',
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error fetching network Ids: ${response.statusText}`)
    }
    return response.json()
  })
}

export function generateRandomNetworkId(
  networkInfo: ControllerNetworkInfo,
): Promise<ControllerNetworkInfo> {
  return fetch(`/ztapi/controller/network`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(networkInfo),
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error fetching network Ids: ${response.statusText}`)
    }
    return response.json()
  })
}

export function getNetworkById(networkId: string): Promise<ControllerNetworkInfo> {
  return fetch(`/ztapi/controller/network/${networkId}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error fetching network by Id: ${response.statusText}`)
    }
    return response.json()
  })
}

export function createOrUpdateNetwork(
  networkId: string,
  controllerinfo: ControllerNetworkInfo,
): Promise<ControllerNetworkInfo> {
  return fetch(`/ztapi/controller/network/${networkId}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(controllerinfo),
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error creating or updating network: ${response.statusText}`)
    }
    return response.json()
  })
}

export function deleteNetwork(networkId: string): Promise<void> {
  return fetch(`/ztapi/controller/network/${networkId}`, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
    },
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error deleting network: ${response.statusText}`)
    }
  })
}

export function listNetworkMemberIds(networkId: string): Promise<Members> {
  return fetch(`/ztapi/controller/network/${networkId}/member`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error fetching network member Ids: ${response.statusText}`)
    }
    return response.json()
  })
}

export function getNetworkMemberById(
  networkId: string,
  memberId: string,
): Promise<ControllerNetworkMemberInfo> {
  return fetch(`/ztapi/controller/network/${networkId}/member/${memberId}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error fetching network member by Id: ${response.statusText}`)
    }
    return response.json()
  })
}

export function createOrUpdateNetworkMember(
  networkId: string,
  memberId: string,
  memberSettings: ControllerNetworkMemberSettings,
): Promise<ControllerNetworkMemberInfo> {
  return fetch(`/ztapi/controller/network/${networkId}/member/${memberId}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(memberSettings),
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error creating or updating network member: ${response.statusText}`)
    }
    return response.json()
  })
}

export function deleteNetworkMember(
  networkId: string,
  memberId: string,
): Promise<ControllerNetworkMemberInfo> {
  return fetch(`/ztapi/controller/network/${networkId}/member/${memberId}`, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
    },
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error deleting network member: ${response.statusText}`)
    }
    return response.json()
  })
}
