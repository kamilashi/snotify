<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import SongInfo from './SongInfo.vue';
import type { Song } from '../stores/song';

const song = ref<Song | null>(null)
const error = ref<string | null>(null)
let es: EventSource | null = null

function connect() {
  es = new EventSource('/snotify/current')

  es.onopen = () => { error.value = null }

  es.onmessage = (e) => {
    song.value = JSON.parse(e.data) as Song | null
    error.value = null
  }

  es.onerror = () => {
    const state = ['connecting', 'open', 'closed'][es!.readyState]
    error.value = `stream error (${state})`
    console.error('SSE error', es!.readyState, es!.url)

    if (es!.readyState === EventSource.CLOSED) {
      es!.close()
      setTimeout(connect, 2000) 
    }
  }
}

onMounted(connect)
onUnmounted(() => es?.close())
</script>

<template>
  <main>
      <p v-if="error" class="error">Error: {{ error }}</p>
      <p v-else-if="!song">Loading…</p>
      <SongInfo v-else :song="song" />
  </main>
</template>

<style scoped>
main {
  font-family: system-ui, sans-serif;
  display: flex;
  margin-top: auto;
  margin-left: auto;
  padding: 0;
  width: 100% ;
  height: 100% ;
  background-color: #ffffff;
}
.error {
  color: #b91c1c;
}
</style>
