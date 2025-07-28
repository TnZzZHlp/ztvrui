export interface LocalNetworkSettings {
  allowDns: boolean
  allowDefault: boolean
  allowManaged: boolean
  allowGlobal: boolean
}

export interface DNS {
  domain: string
  servers: string[]
}

export interface MulticastSubscription {
  adi: number
  mac: string
}

export interface Route {
  flags: number
  metric: number
  target: string
  via: string
}

export interface LocalNetworkInfo {
  allowDNS: boolean
  allowDefault: boolean
  allowManaged: boolean
  allowGlobal: boolean
  assignedAddresses: string[]
  bridge: boolean
  broadcastEnabled: boolean
  dns: DNS
  id: string
  mac: string
  mtu: number
  multicastSubscriptions: MulticastSubscription[]
  authenticationURL: string
  authenticationExpiryTime: number
  name: string
  netconfRevision: number
  portDeviceName: string
  portError: number
  routes: Route[]
  status: string
  type: string
}
