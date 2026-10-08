<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface UserData {
  key : string,
  value : string
}

interface Song {
  name: string | null,
  artist: string | null,
  duration_ms: number | null,
  user_data: UserData[], 
}

const song = ref<Song | null>(null)
const error = ref<string | null>(null)
let es: EventSource | null = null

onMounted(() => {
  es = new EventSource('/snotify/current')
  es.onmessage = (e) => {
    song.value = JSON.parse(e.data) as Song | null
    error.value = null
  }
  es.onerror = () => {
    error.value = 'connection lost, retrying…'
  }
})

onUnmounted(() => es?.close())
</script>

<template>
  <main>
    <p v-if="error" class="error">Error: {{ error }}</p>
    <p v-else-if="!song">Loading…</p>
    <template v-else>
      <h1 class="headline">{{ song.name }}</h1>
      <p>Artist: <span class="row">{{ song.artist }}</span></p>
      <p>Duration: <span class="row">{{ song.duration_ms }}</span></p>
      <li
        v-for="pair in song.user_data"
        :key="pair.key"
        class="flex items-center gap-2"
      >
      <span>{{ pair.key }}</span>:<span>{{ pair.value }}</span>
    </li>
    </template>
  </main>
</template>

<style scoped>
main {
  font-family: system-ui, sans-serif;
  margin: 0;
  padding: 0;
  width: 100% ;
  height: 100% ;
  background-color: #ffffff;
}
.error {
  color: #b91c1c;
}
.headline {
  color: #d2dcf2;
  background-color: #0945d3;
}
.row {
  font-variant-numeric: tabular-nums;
  color: #0369a1;
}
</style>
