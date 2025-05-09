<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

async function previous_track() {}

async function toggle_playback() {
  invoke("toggle_audio");
}

async function next_track() {}
</script>

<template>
  <footer class="w-full border-t border-gray-200 py-10 text-center">
    <div class="flex justify-center gap-4">
      <UButton icon="i-lucide-skip-back" @click="previous_track" />
      <UButton icon="i-lucide-play" @click="toggle_playback" />
      <UButton icon="i-lucide-skip-forward" @click="next_track" />
    </div>
  </footer>
</template>
