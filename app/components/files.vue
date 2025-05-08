<script setup lang="ts">
import { ref, computed, h, resolveComponent } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { TableColumn, TableRow } from "@nuxt/ui";

// Optional Nuxt UI button for sortable header icon
const UButton = resolveComponent("UButton");

type FileRow = {
  // index: number;
  name: string;
  fullPath: string;
};

const items = ref<string[]>([]);

async function get_files(filePath: string) {
  const files: string[] = await invoke("read_music_files", {
    filePath,
    allowedFormats: null,
  });

  return files;
}

onMounted(async () => {
  // const files = await get_files(hardcodedPath);
  // items.value = files;
});

async function file_dialog() {
  const folder = await open({
    multiple: false,
    directory: true,
  });

  if (folder === null || typeof folder !== "string") return;

  const files = await get_files(folder);
  items.value = files;
}

const data = computed<FileRow[]>(() =>
  items.value.map((fullPath, _index) => {
    const fileName = fullPath.split("/").pop() || fullPath;
    return {
      name: fileName,
      fullPath,
    };
  })
);

const sorting = ref([{ id: "name", desc: false }]);

const columns: TableColumn<FileRow>[] = [
  // {
  //   accessorKey: "index",
  //   header: "#",
  //   enableSorting: false,
  // },
  {
    accessorKey: "name",
    header: ({ column }) => {
      const isSorted = column.getIsSorted();

      return h(UButton, {
        color: "neutral",
        variant: "ghost",
        label: "Filename",
        icon: isSorted
          ? isSorted === "asc"
            ? "i-lucide-arrow-up-narrow-wide"
            : "i-lucide-arrow-down-wide-narrow"
          : "i-lucide-arrow-up-down",
        class: "-mx-2.5",
        onClick: () => column.toggleSorting(isSorted === "asc"),
      });
    },
  },
];

const rowSelection = ref<Record<string, boolean>>({});

function onSelect(row: TableRow<FileRow>, _e?: Event) {
  row.toggleSelected(!row.getIsSelected());
  console.log("Selected row:", row.original);
}
</script>

<template>
  <div class="p-4 space-y-4 h-screen flex flex-col w-full">
    <div class="w-full flex justify-end">
      <UButton label="Add folder" icon="i-lucide-plus" @click="file_dialog" />
    </div>

    <div class="flex-1 overflow-auto border rounded w-full">
      <UTable
        ref="table"
        v-model:sorting="sorting"
        v-model:row-selection="rowSelection"
        :data="data"
        :columns="columns"
        @select="onSelect"
        class="w-full"
      />
    </div>
  </div>
</template>
