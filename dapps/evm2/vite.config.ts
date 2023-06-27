import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { NodeGlobalsPolyfillPlugin } from '@esbuild-plugins/node-globals-polyfill'

export default defineConfig({
  plugins: [react()],
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: "globalThis",
      },
      plugins: [
        NodeGlobalsPolyfillPlugin({
            buffer: true
        })
    ]
    },
  },
  server: {
    port: 5000,
  },
  resolve: {
    alias: {
      process: "process",
      stream: "stream-browserify",
      zlib: "browserify-zlib",
      util: "util",
    },
  },
});