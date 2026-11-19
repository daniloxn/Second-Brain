import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// Porta fixa: o Tauri espera o devUrl nesta porta (ver src-tauri/tauri.conf.json).
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1430,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: "es2021",
    outDir: "dist",
    emptyOutDir: true,
    // App de desktop: os chunks carregam do disco, então o limite de 500 kB do navegador não se aplica.
    chunkSizeWarningLimit: 1000,
  },
});
