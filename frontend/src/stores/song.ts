export interface UserData {
  key: string
  value: string
}

export interface Song {
  name: string | null
  artist: string | null
  duration_ms: number | null
  user_data: UserData[]
}
