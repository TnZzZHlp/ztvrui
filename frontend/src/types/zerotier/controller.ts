export interface ControllerStatusInfo {
  controller: boolean
  apiVersion: number
  clock: number
  databaseReady: boolean
}

export interface ControllerNetworkInfo {
  name: string
  enableBroadcast?: boolean
  mtu?: number
  dns?: DNS
  private?: boolean
  ipAssignmentPools?: IPAssignmentPool[]
  v4AssignMode?: V4AssignMode
  v6AssignMode?: V6AssignMode
  multicastLimit?: number
  routes?: Route[]
  id?: string
  nwid?: string
  creationTime?: number
  objtype?: string
  revision?: number
  capabilities?: Array<number[]>
  rules?: Rule[]
  tags?: Tag[]
}

export interface Tag {
  id: string
  default?: string | null
}

export interface DNS {
  domain: string
  servers: string[]
}

export interface IPAssignmentPool {
  ipRangeStart: string
  ipRangeEnd: string
}

export interface Route {
  target: string
  via: string
}

export interface Rule {
  not: boolean
  or: boolean
  type: string
}

export interface V4AssignMode {
  zt: boolean
}

export interface V6AssignMode {
  '6plane': boolean
  rfc4193: boolean
  zt: boolean
}

export interface Members {
  [memberId: string]: number
}

export interface ControllerNetworkMemberInfo {
  id: string
  authorized: boolean
  activeBridge: boolean
  ipAssignments: string[]
  name: string
  noAutoAssignIps: boolean
  ssoExempt: boolean
  address: string
  authenticationExpiryTime: number
  capabilities: number[]
  creationTime: number
  identity: string
  lastAuthorizedCredential: string
  lastAuthorizedCredentialType: string
  lastAuthorizedTime: number
  lastDeauthorizedTime: number
  nwid: string
  objtype: string
  remoteTraceLevel: number
  remoteTraceTarget: string
  revision: number
  tags: null
  vMajor: number
  vMinor: number
  vProto: number
  vRev: number
}

export interface ControllerNetworkMemberSettings {
  authorized: boolean
  activeBridge: boolean
  ipAssignments: string[]
  name: string
  noAutoAssignIps: boolean
  ssoExempt: boolean
}
