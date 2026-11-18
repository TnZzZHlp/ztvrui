const MEMBER_ID_REGEX = /^[0-9a-fA-F]{10}$/
const UNSIGNED_INT_REGEX = /^\d+$/

export function isValidIPv4(ip: string): boolean {
  const parts = ip.split('.')
  if (parts.length !== 4) return false

  return parts.every((part) => {
    if (!UNSIGNED_INT_REGEX.test(part)) return false
    if (part.length > 1 && part.startsWith('0')) return false
    const value = Number(part)
    return value >= 0 && value <= 255
  })
}

export function isValidIPv6(ip: string): boolean {
  if (!ip || ip.includes('%') || ip.includes('[') || ip.includes(']') || !ip.includes(':')) {
    return false
  }
  try {
    const parsed = new URL(`http://[${ip}]/`)
    return parsed.hostname.startsWith('[') && parsed.hostname.endsWith(']')
  } catch {
    return false
  }
}

export function isValidIP(ip: string): boolean {
  return isValidIPv4(ip) || isValidIPv6(ip)
}

function ipv4ToUint32(ip: string): number {
  const [a, b, c, d] = ip.split('.').map(Number)
  return (((a << 24) >>> 0) + (b << 16) + (c << 8) + d) >>> 0
}

function expandIPv6ToBigInt(ip: string): bigint | null {
  if (!isValidIPv6(ip)) return null
  const normalized = new URL(`http://[${ip}]/`).hostname.slice(1, -1)
  const [leftRaw, rightRaw] = normalized.split('::')
  if (normalized.split('::').length > 2) return null

  const parseSide = (side: string): number[] | null => {
    if (!side) return []
    const tokens = side.split(':')
    const result: number[] = []
    for (let i = 0; i < tokens.length; i++) {
      const token = tokens[i]
      if (token.includes('.')) {
        if (i !== tokens.length - 1 || !isValidIPv4(token)) return null
        const octets = token.split('.').map(Number)
        result.push((octets[0] << 8) | octets[1], (octets[2] << 8) | octets[3])
      } else {
        result.push(parseInt(token, 16))
      }
    }
    return result
  }

  const left = parseSide(leftRaw ?? '')
  const right = rightRaw !== undefined ? parseSide(rightRaw) : []
  if (!left || !right) return null

  const missing = 8 - (left.length + right.length)
  if (missing < 0) return null

  const groups = [...left, ...Array<number>(missing).fill(0), ...right]
  if (groups.length !== 8) return null

  return groups.reduce((acc, g) => (acc << 16n) | BigInt(g), 0n)
}

export function isValidIPPoolRange(start: string, end: string): boolean {
  if (isValidIPv4(start) && isValidIPv4(end)) {
    return ipv4ToUint32(start) <= ipv4ToUint32(end)
  }

  const startV6 = expandIPv6ToBigInt(start)
  const endV6 = expandIPv6ToBigInt(end)
  if (startV6 !== null && endV6 !== null) {
    return startV6 <= endV6
  }

  return false
}

export function isValidCIDR(cidr: string): boolean {
  const parts = cidr.split('/')
  if (parts.length !== 2) return false

  const [ip, prefixStr] = parts
  if (!UNSIGNED_INT_REGEX.test(prefixStr)) return false
  if (prefixStr.length > 1 && prefixStr.startsWith('0')) return false

  const prefix = Number(prefixStr)
  if (isValidIPv4(ip)) {
    return prefix >= 0 && prefix <= 32
  }
  if (isValidIPv6(ip)) {
    return prefix >= 0 && prefix <= 128
  }
  return false
}

export function isValidMemberId(id: string): boolean {
  return MEMBER_ID_REGEX.test(id.trim())
}
