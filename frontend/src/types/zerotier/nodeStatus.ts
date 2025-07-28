export interface NodeStatus {
  address: string
  clock: number
  config: Config
  online: boolean
  planetWorldId: number
  planetWorldTimestamp: number
  publicIdentity: string
  tcpFallbackActive: boolean
  version: string
  versionBuild: number
  versionMajor: number
  versionMinor: number
  versionRev: number
}

export interface Config {
  settings: Settings
}

export interface Settings {
  allowManagementFrom: string[]
  allowTcpFallbackRelay: boolean
  forceTcpRelay: boolean
  listeningOn: string[]
  portMappingEnabled: boolean
  primaryPort: number
  secondaryPort: number
  softwareUpdate: string
  softwareUpdateChannel: string
  surfaceAddresses: string[]
  tertiaryPort: number
}
