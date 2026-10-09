<script setup lang="ts">
import type { Song } from '../stores/song';

interface Props {
  song: Song;
}

const props = defineProps<Props>();
</script>

<template>
  <div class="container">
    <h1 class="headline">{{ song.name }}</h1>
    <span class="row">Artist: {{ song.artist }}</span>
    <span class="row">Duration: {{ song.duration_ms }}</span>
    <li
        v-for="pair in song.user_data"
        :key="pair.key"
        class="flex items-center gap-2"
      >
      <span class="row">{{ pair.key }}: {{ pair.value }}</span>
    </li>
</div>
</template>

<style scoped>
.container {
 display: flex;
 flex-direction: column; 
 background-color: #d35009;
 max-width: 300pt;
 border-radius: 5pt;
 padding: 20pt;
}
.headline {
  color: #d2dcf2;
}
.row {
  font-variant-numeric: tabular-nums;
  color: #bbe7f1;
}
</style>
