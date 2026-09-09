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
      <h1>{{ message.text }}</h1>
      <p>Count: {{ message.count }}</p>
    </template>
  </main>
</template>

<style scoped>
main {
  font-family: system-ui, sans-serif;
  padding: 2rem;
  max-width: 40rem;
}
.error {
  color: #b91c1c;
}
</style>
