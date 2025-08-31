interface TokenData {
  token: string
  expiresAt: number
  username: string
}

class TokenManager {
  private readonly TOKEN_KEY = 'jwt_token'
  private readonly EXPIRES_KEY = 'jwt_expires'
  private readonly USERNAME_KEY = 'jwt_username'

  setToken(token: string, expiresAt: number, username: string): void {
    localStorage.setItem(this.TOKEN_KEY, token)
    localStorage.setItem(this.EXPIRES_KEY, expiresAt.toString())
    localStorage.setItem(this.USERNAME_KEY, username)
  }

  getToken(): string | null {
    return localStorage.getItem(this.TOKEN_KEY)
  }

  getUsername(): string | null {
    return localStorage.getItem(this.USERNAME_KEY)
  }

  getExpiresAt(): number | null {
    const expires = localStorage.getItem(this.EXPIRES_KEY)
    return expires ? parseInt(expires) : null
  }

  isTokenExpired(): boolean {
    const expiresAt = this.getExpiresAt()
    if (!expiresAt) return true

    // Add 5 minute buffer before expiration
    const now = Math.floor(Date.now() / 1000)
    return now >= expiresAt - 300
  }

  clearToken(): void {
    localStorage.removeItem(this.TOKEN_KEY)
    localStorage.removeItem(this.EXPIRES_KEY)
    localStorage.removeItem(this.USERNAME_KEY)
  }

  getAuthHeader(): Record<string, string> {
    const token = this.getToken()
    return token ? { Authorization: `Bearer ${token}` } : {}
  }

  getTokenData(): TokenData | null {
    const token = this.getToken()
    const expiresAt = this.getExpiresAt()
    const username = this.getUsername()

    if (!token || !expiresAt || !username) {
      return null
    }

    return { token, expiresAt, username }
  }
}

export const tokenManager = new TokenManager()
