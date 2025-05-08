import { defineNuxtConfig } from "nuxt/config";

export default defineNuxtConfig({
  modules: [
    "@nuxt/ui",
    "@nuxt/eslint",
    "@nuxt/fonts",
    "@nuxt/icon",
    "@nuxt/image",
    "@nuxt/eslint",
    "@vueuse/nuxt",
  ],
  app: {
    head: {
      title: "drakn",
      charset: "utf-8",
      viewport: "width=device-width, initial-scale=1",
      meta: [{ name: "format-detection", content: "no" }],
    },
    pageTransition: {
      name: "page",
      mode: "out-in",
    },
    layoutTransition: {
      name: "layout",
      mode: "out-in",
    },
  },
  css: ["~/assets/css/main.css"],

  ssr: false,
  dir: {
    modules: "app/modules",
  },
  imports: {
    presets: [
      {
        from: "zod",
        imports: [
          "z",
          {
            name: "infer",
            as: "zInfer",
            type: true,
          },
        ],
      },
    ],
  },
  vite: {
    clearScreen: false,
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      strictPort: true,
      hmr: {
        protocol: "ws",
        host: "localhost",
        port: 1420,
      },
      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },
  },
  devServer: {
    host: "localhost",
    port: 1420,
  },
  router: {
    options: {
      scrollBehaviorType: "smooth",
    },
  },
  experimental: {
    typedPages: true,
  },
  future: {
    compatibilityVersion: 4,
  },
  compatibilityDate: "2025-04-01",
});
