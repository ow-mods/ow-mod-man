import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig({
    publicDir: false,
    clearScreen: false,
    server: {
        strictPort: true
    },
    envPrefix: ["VITE_", "TAURI_"],
    plugins: [react()],
    build: {
        outDir: "../../dist",
        target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
        minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
        sourcemap: !!process.env.TAURI_DEBUG
    },
    resolve: {
        alias: [
            { find: "@components", replacement: path.resolve(__dirname, "src/components") },
            { find: "@styles", replacement: path.resolve(__dirname, "src/styles") },
            { find: "@assets", replacement: path.resolve(__dirname, "src/assets") },
            { find: "@types", replacement: path.resolve(__dirname, "src/types.ts") }
        ]
    }
});
