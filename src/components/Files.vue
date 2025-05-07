<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

async function get_files(filePath: string) {
  const files = await invoke("read_music_files", {
    filePath: filePath,
    allowedFormats: null,
  });
  console.log(files);
}
async function file_dialog() {
  const folder = await open({
    multiple: false,
    directory: true,
  });

  if (folder === null) {
    return;
  }

  await get_files(folder);
}
</script>

<template>
  <main class="container">
    <button @click="file_dialog">Select file</button>
  </main>
</template>
