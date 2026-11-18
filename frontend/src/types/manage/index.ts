export interface Auth {
  username: string
  password: string
}

export interface UpdateProfilePayload {
  current_password: string
  username: string
  password: string
}

export interface UpdateProfileResponse {
  message: string
  token: string
  expires_at: number
  username: string
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
