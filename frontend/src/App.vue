<script setup lang="ts">
import { ref, onMounted } from 'vue'

interface Message {
  text: string
  count: number
}

const message = ref<Message | null>(null)
const error = ref<string | null>(null)

onMounted(async () => {
  try {
    const res = await fetch('/api/hello')
    if (!res.ok) throw new Error(`HTTP ${res.status}`)
    message.value = (await res.json()) as Message
  } catch (e: unknown) {
    error.value = e instanceof Error ? e.message : 'request failed'
  }
})
</script>

<template>
  <main>
    <p v-if="error" class="error">Error: {{ error }}</p>
    <p v-else-if="!message">Loading…</p>
    <template v-else>
      <h1 class="headline">{{ message.text }}</h1>
      <p>Count: <span class="count">{{ message.count }}</span></p>
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
.count {
  font-variant-numeric: tabular-nums;
  color: #0369a1;
}
</style>
