import { defineConfig } from "vite";
import { createHtmlPlugin } from "vite-plugin-html";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig({
    publicDir: false,
    clearScreen: false,
    server: {
        strictPort: true
    },
    envPrefix: ["VITE_", "TAURI_"],
    plugins: [
        react(),
        createHtmlPlugin({
            minify: true,
            template: "index.html",
            inject: {
                data: {
                    reactDevTools: process.env.TAURI_DEBUG
                        ? '<script src="http://localhost:8097"></script>'
                        : ""
                }
            }
        })
    ],
    build: {
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
            { find: "@types", replacement: path.resolve(__dirname, "./src/types.ts") },
            { find: "@hooks", replacement: path.resolve(__dirname, "./src/hooks.ts") }
        ]
    }
});
