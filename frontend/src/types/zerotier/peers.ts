export interface PeerInfo {
  address: string
  isBonded: boolean
  latency: number
  paths: Path[]
  role: string
  version: string
  versionMajor: number
  versionMinor: number
  versionRev: number
  tunneled: boolean
}

export interface Path {
  active: boolean
  address: string
  expired: boolean
  lastReceive: number
  lastSend: number
  localSocket: number
  preferred: boolean
  trustedPathId: number
}
