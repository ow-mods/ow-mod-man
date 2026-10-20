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
                main: path.resolve(import.meta.dirname, "./index.html"),
                logs: path.resolve(import.meta.dirname, "./logs/index.html")
            }
        },
        outDir: "../dist",
        minify: !process.env.TAURI_ENV_DEBUG,
        sourcemap: !!process.env.TAURI_ENV_DEBUG
    },
    resolve: {
        alias: [
            {
                find: "@components",
                replacement: path.resolve(import.meta.dirname, "./src/components")
            },
            { find: "@styles", replacement: path.resolve(import.meta.dirname, "./src/styles") },
            { find: "@assets", replacement: path.resolve(import.meta.dirname, "./src/assets") },
            { find: "@types", replacement: path.resolve(import.meta.dirname, "./src/types.d.ts") },
            { find: "@hooks", replacement: path.resolve(import.meta.dirname, "./src/hooks.ts") },
            {
                find: "@commands",
                replacement: path.resolve(import.meta.dirname, "./src/commands.ts")
            },
            { find: "@events", replacement: path.resolve(import.meta.dirname, "./src/events.ts") }
        ]
    }
});
