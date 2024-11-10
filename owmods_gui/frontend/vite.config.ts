/// <reference types="vite/client" />

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { imagetools } from "vite-imagetools";

const host = process.env.TAURI_DEV_HOST;

import path from "path";

export default defineConfig({
    publicDir: false,
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                  protocol: "ws",
                  host,
                  port: 1421
              }
            : undefined,
        watch: {
            ignored: ["owmods_gui/backend/**"]
        }
    },
    envPrefix: ["VITE_", "TAURI_ENV_"],
    plugins: [react({ include: process.cwd() }), imagetools()],
    build: {
        rollupOptions: {
            input: {
                main: path.resolve(__dirname, "./index.html"),
                logs: path.resolve(__dirname, "./logs/index.html")
            }
        },
        outDir: "../dist",
        target: process.env.TAURI_ENV_PLATFORM == "windows" ? "chrome105" : "safari13",
        minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
        sourcemap: !!process.env.TAURI_ENV_DEBUG
    },
    resolve: {
        alias: [
            { find: "@components", replacement: path.resolve(__dirname, "./src/components") },
            { find: "@styles", replacement: path.resolve(__dirname, "./src/styles") },
            { find: "@assets", replacement: path.resolve(__dirname, "./src/assets") },
            { find: "@types", replacement: path.resolve(__dirname, "./src/types.d.ts") },
            { find: "@hooks", replacement: path.resolve(__dirname, "./src/hooks.ts") },
            { find: "@commands", replacement: path.resolve(__dirname, "./src/commands.ts") },
            { find: "@events", replacement: path.resolve(__dirname, "./src/events.ts") }
        ]
    }
});
