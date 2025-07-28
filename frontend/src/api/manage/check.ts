export function check(): Promise<void> {
  return fetch(`/api/check`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  }).then((response) => {
    if (!response.ok) {
      throw new Error(`Error checking manage status: ${response.statusText}`)
    }
  })
}
