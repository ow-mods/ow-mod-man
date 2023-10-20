/// <reference types="vitest" />
/// <reference types="vite/client" />

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { createHtmlPlugin } from "vite-plugin-html";
import { imagetools } from "vite-imagetools";

import path from "path";

export default defineConfig({
    publicDir: false,
    clearScreen: false,
    server: {
        strictPort: true
    },
    envPrefix: [
        "VITE_",
        "TAURI_PLATFORM",
        "TAURI_ARCH",
        "TAURI_FAMILY",
        "TAURI_PLATFORM_VERSION",
        "TAURI_PLATFORM_TYPE",
        "TAURI_DEBUG"
    ],
    plugins: [react(), imagetools(), createHtmlPlugin({ minify: true })],
    build: {
        rollupOptions: {
            input: {
                main: path.resolve(__dirname, "index.html"),
                logs: path.resolve(__dirname, "logs/index.html")
            }
        },
        outDir: "../dist",
        target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
        minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
        sourcemap: !!process.env.TAURI_DEBUG
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
    },
    test: {
        globals: true,
        environment: "jsdom",
        watch: false,
        setupFiles: ["./src/tests/setup.ts"],
        onConsoleLog: (msg) => {
            if (msg.includes("window.__TAURI_METADATA__")) {
                return false;
            }
        }
    }
});
