<script setup lang="ts">
import type { Song } from '../stores/song';

interface Props {
  song: Song;
}

const props = defineProps<Props>();
</script>

<template>
  <div class="container">
    <Transition name="fade" mode="out-in">
    <div class="content" :key="`${song.artist}-${song.name}`">
        <h1 class="headline">{{ song.name }}</h1>
        <span class="row">Artist: {{ song.artist }}</span>
        <span v-if="song.duration_ms" class="row">Duration: {{ song.duration_ms }}</span>
        <span class="row" 
        v-for="pair in song.user_data" 
            :key="pair.key">
            {{ pair.key }}: {{ pair.value }}
        </span>
    </div>
    </Transition> 
  </div>
</template>

<style scoped>
.container {
 display: flex;
 background-color: #d35009;
 width: 390pt;
 height: 150pt;
 border-radius: 5pt;
 padding: 20pt;
 padding-right: 50pt;
}
.content {
 display: flex;
 flex-direction: column; 
 width: 100%;
 width: 100%;
 padding: 0;
 margin: 0;
}
.headline {
  display: flex;
  flex-direction: row; 
  justify-content: flex-end;
  color: #d2dcf2;
  font-size: 2.5rem;
}
.row {
  display: flex;
  flex-direction: row; 
  justify-content: flex-end;
  font-variant-numeric: tabular-nums;
  color: #bbe7f1;
  font-size: 1.1rem;
}
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
