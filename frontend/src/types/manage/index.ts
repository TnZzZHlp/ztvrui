export interface Auth {
  username: string
  password: string
}

export interface LoginResponse {
  token: string
  message: string
  expires_at: number
  username: string
}

export interface RefreshResponse {
  token: string
  expires_at: number
  message: string
}
