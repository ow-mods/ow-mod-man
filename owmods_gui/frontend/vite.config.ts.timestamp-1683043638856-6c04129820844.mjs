// vite.config.ts
import { defineConfig } from "file:///C:/Users/bencro855/Documents/GitHub/ow-mod-man/owmods_gui/frontend/node_modules/.pnpm/vite@4.3.1_sass@1.62.0/node_modules/vite/dist/node/index.js";
import { createHtmlPlugin } from "file:///C:/Users/bencro855/Documents/GitHub/ow-mod-man/owmods_gui/frontend/node_modules/.pnpm/vite-plugin-html@3.2.0_vite@4.3.1/node_modules/vite-plugin-html/dist/index.mjs";
import react from "file:///C:/Users/bencro855/Documents/GitHub/ow-mod-man/owmods_gui/frontend/node_modules/.pnpm/@vitejs+plugin-react@3.1.0_vite@4.3.1/node_modules/@vitejs/plugin-react/dist/index.mjs";
import path from "path";
var __vite_injected_original_dirname =
    "C:\\Users\\bencro855\\Documents\\GitHub\\ow-mod-man\\owmods_gui\\frontend";
var vite_config_default = defineConfig({
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
        rollupOptions: {
            input: {
                main: path.resolve(__vite_injected_original_dirname, "index.html"),
                logs: path.resolve(__vite_injected_original_dirname, "logs/index.html")
            }
        },
        outDir: "../dist",
        target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
        minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
        sourcemap: !!process.env.TAURI_DEBUG
    },
    resolve: {
        alias: [
            {
                find: "@components",
                replacement: path.resolve(__vite_injected_original_dirname, "./src/components")
            },
            {
                find: "@styles",
                replacement: path.resolve(__vite_injected_original_dirname, "./src/styles")
            },
            {
                find: "@assets",
                replacement: path.resolve(__vite_injected_original_dirname, "./src/assets")
            },
            {
                find: "@types",
                replacement: path.resolve(__vite_injected_original_dirname, "./src/types.d.ts")
            },
            {
                find: "@hooks",
                replacement: path.resolve(__vite_injected_original_dirname, "./src/hooks.ts")
            },
            {
                find: "@commands",
                replacement: path.resolve(__vite_injected_original_dirname, "./src/commands.ts")
            }
        ]
    }
});
export { vite_config_default as default };
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJDOlxcXFxVc2Vyc1xcXFxiZW5jcm84NTVcXFxcRG9jdW1lbnRzXFxcXEdpdEh1YlxcXFxvdy1tb2QtbWFuXFxcXG93bW9kc19ndWlcXFxcZnJvbnRlbmRcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIkM6XFxcXFVzZXJzXFxcXGJlbmNybzg1NVxcXFxEb2N1bWVudHNcXFxcR2l0SHViXFxcXG93LW1vZC1tYW5cXFxcb3dtb2RzX2d1aVxcXFxmcm9udGVuZFxcXFx2aXRlLmNvbmZpZy50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vQzovVXNlcnMvYmVuY3JvODU1L0RvY3VtZW50cy9HaXRIdWIvb3ctbW9kLW1hbi9vd21vZHNfZ3VpL2Zyb250ZW5kL3ZpdGUuY29uZmlnLnRzXCI7aW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSBcInZpdGVcIjtcbmltcG9ydCB7IGNyZWF0ZUh0bWxQbHVnaW4gfSBmcm9tIFwidml0ZS1wbHVnaW4taHRtbFwiO1xuaW1wb3J0IHJlYWN0IGZyb20gXCJAdml0ZWpzL3BsdWdpbi1yZWFjdFwiO1xuaW1wb3J0IHBhdGggZnJvbSBcInBhdGhcIjtcblxuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKHtcbiAgICBwdWJsaWNEaXI6IGZhbHNlLFxuICAgIGNsZWFyU2NyZWVuOiBmYWxzZSxcbiAgICBzZXJ2ZXI6IHtcbiAgICAgICAgc3RyaWN0UG9ydDogdHJ1ZVxuICAgIH0sXG4gICAgZW52UHJlZml4OiBbXCJWSVRFX1wiLCBcIlRBVVJJX1wiXSxcbiAgICBwbHVnaW5zOiBbXG4gICAgICAgIHJlYWN0KCksXG4gICAgICAgIGNyZWF0ZUh0bWxQbHVnaW4oe1xuICAgICAgICAgICAgbWluaWZ5OiB0cnVlLFxuICAgICAgICAgICAgdGVtcGxhdGU6IFwiaW5kZXguaHRtbFwiLFxuICAgICAgICAgICAgaW5qZWN0OiB7XG4gICAgICAgICAgICAgICAgZGF0YToge1xuICAgICAgICAgICAgICAgICAgICByZWFjdERldlRvb2xzOiBwcm9jZXNzLmVudi5UQVVSSV9ERUJVR1xuICAgICAgICAgICAgICAgICAgICAgICAgPyAnPHNjcmlwdCBzcmM9XCJodHRwOi8vbG9jYWxob3N0OjgwOTdcIj48L3NjcmlwdD4nXG4gICAgICAgICAgICAgICAgICAgICAgICA6IFwiXCJcbiAgICAgICAgICAgICAgICB9XG4gICAgICAgICAgICB9XG4gICAgICAgIH0pXG4gICAgXSxcbiAgICBidWlsZDoge1xuICAgICAgICByb2xsdXBPcHRpb25zOiB7XG4gICAgICAgICAgICBpbnB1dDoge1xuICAgICAgICAgICAgICAgIG1haW46IHBhdGgucmVzb2x2ZShfX2Rpcm5hbWUsIFwiaW5kZXguaHRtbFwiKSxcbiAgICAgICAgICAgICAgICBsb2dzOiBwYXRoLnJlc29sdmUoX19kaXJuYW1lLCBcImxvZ3MvaW5kZXguaHRtbFwiKVxuICAgICAgICAgICAgfVxuICAgICAgICB9LFxuICAgICAgICBvdXREaXI6IFwiLi4vZGlzdFwiLFxuICAgICAgICB0YXJnZXQ6IHByb2Nlc3MuZW52LlRBVVJJX1BMQVRGT1JNID09IFwid2luZG93c1wiID8gXCJjaHJvbWUxMDVcIiA6IFwic2FmYXJpMTNcIixcbiAgICAgICAgbWluaWZ5OiAhcHJvY2Vzcy5lbnYuVEFVUklfREVCVUcgPyBcImVzYnVpbGRcIiA6IGZhbHNlLFxuICAgICAgICBzb3VyY2VtYXA6ICEhcHJvY2Vzcy5lbnYuVEFVUklfREVCVUdcbiAgICB9LFxuICAgIHJlc29sdmU6IHtcbiAgICAgICAgYWxpYXM6IFtcbiAgICAgICAgICAgIHsgZmluZDogXCJAY29tcG9uZW50c1wiLCByZXBsYWNlbWVudDogcGF0aC5yZXNvbHZlKF9fZGlybmFtZSwgXCIuL3NyYy9jb21wb25lbnRzXCIpIH0sXG4gICAgICAgICAgICB7IGZpbmQ6IFwiQHN0eWxlc1wiLCByZXBsYWNlbWVudDogcGF0aC5yZXNvbHZlKF9fZGlybmFtZSwgXCIuL3NyYy9zdHlsZXNcIikgfSxcbiAgICAgICAgICAgIHsgZmluZDogXCJAYXNzZXRzXCIsIHJlcGxhY2VtZW50OiBwYXRoLnJlc29sdmUoX19kaXJuYW1lLCBcIi4vc3JjL2Fzc2V0c1wiKSB9LFxuICAgICAgICAgICAgeyBmaW5kOiBcIkB0eXBlc1wiLCByZXBsYWNlbWVudDogcGF0aC5yZXNvbHZlKF9fZGlybmFtZSwgXCIuL3NyYy90eXBlcy5kLnRzXCIpIH0sXG4gICAgICAgICAgICB7IGZpbmQ6IFwiQGhvb2tzXCIsIHJlcGxhY2VtZW50OiBwYXRoLnJlc29sdmUoX19kaXJuYW1lLCBcIi4vc3JjL2hvb2tzLnRzXCIpIH0sXG4gICAgICAgICAgICB7IGZpbmQ6IFwiQGNvbW1hbmRzXCIsIHJlcGxhY2VtZW50OiBwYXRoLnJlc29sdmUoX19kaXJuYW1lLCBcIi4vc3JjL2NvbW1hbmRzLnRzXCIpIH1cbiAgICAgICAgXVxuICAgIH1cbn0pO1xuIl0sCiAgIm1hcHBpbmdzIjogIjtBQUF3WSxTQUFTLG9CQUFvQjtBQUNyYSxTQUFTLHdCQUF3QjtBQUNqQyxPQUFPLFdBQVc7QUFDbEIsT0FBTyxVQUFVO0FBSGpCLElBQU0sbUNBQW1DO0FBS3pDLElBQU8sc0JBQVEsYUFBYTtBQUFBLEVBQ3hCLFdBQVc7QUFBQSxFQUNYLGFBQWE7QUFBQSxFQUNiLFFBQVE7QUFBQSxJQUNKLFlBQVk7QUFBQSxFQUNoQjtBQUFBLEVBQ0EsV0FBVyxDQUFDLFNBQVMsUUFBUTtBQUFBLEVBQzdCLFNBQVM7QUFBQSxJQUNMLE1BQU07QUFBQSxJQUNOLGlCQUFpQjtBQUFBLE1BQ2IsUUFBUTtBQUFBLE1BQ1IsVUFBVTtBQUFBLE1BQ1YsUUFBUTtBQUFBLFFBQ0osTUFBTTtBQUFBLFVBQ0YsZUFBZSxRQUFRLElBQUksY0FDckIsa0RBQ0E7QUFBQSxRQUNWO0FBQUEsTUFDSjtBQUFBLElBQ0osQ0FBQztBQUFBLEVBQ0w7QUFBQSxFQUNBLE9BQU87QUFBQSxJQUNILGVBQWU7QUFBQSxNQUNYLE9BQU87QUFBQSxRQUNILE1BQU0sS0FBSyxRQUFRLGtDQUFXLFlBQVk7QUFBQSxRQUMxQyxNQUFNLEtBQUssUUFBUSxrQ0FBVyxpQkFBaUI7QUFBQSxNQUNuRDtBQUFBLElBQ0o7QUFBQSxJQUNBLFFBQVE7QUFBQSxJQUNSLFFBQVEsUUFBUSxJQUFJLGtCQUFrQixZQUFZLGNBQWM7QUFBQSxJQUNoRSxRQUFRLENBQUMsUUFBUSxJQUFJLGNBQWMsWUFBWTtBQUFBLElBQy9DLFdBQVcsQ0FBQyxDQUFDLFFBQVEsSUFBSTtBQUFBLEVBQzdCO0FBQUEsRUFDQSxTQUFTO0FBQUEsSUFDTCxPQUFPO0FBQUEsTUFDSCxFQUFFLE1BQU0sZUFBZSxhQUFhLEtBQUssUUFBUSxrQ0FBVyxrQkFBa0IsRUFBRTtBQUFBLE1BQ2hGLEVBQUUsTUFBTSxXQUFXLGFBQWEsS0FBSyxRQUFRLGtDQUFXLGNBQWMsRUFBRTtBQUFBLE1BQ3hFLEVBQUUsTUFBTSxXQUFXLGFBQWEsS0FBSyxRQUFRLGtDQUFXLGNBQWMsRUFBRTtBQUFBLE1BQ3hFLEVBQUUsTUFBTSxVQUFVLGFBQWEsS0FBSyxRQUFRLGtDQUFXLGtCQUFrQixFQUFFO0FBQUEsTUFDM0UsRUFBRSxNQUFNLFVBQVUsYUFBYSxLQUFLLFFBQVEsa0NBQVcsZ0JBQWdCLEVBQUU7QUFBQSxNQUN6RSxFQUFFLE1BQU0sYUFBYSxhQUFhLEtBQUssUUFBUSxrQ0FBVyxtQkFBbUIsRUFBRTtBQUFBLElBQ25GO0FBQUEsRUFDSjtBQUNKLENBQUM7IiwKICAibmFtZXMiOiBbXQp9Cg==
