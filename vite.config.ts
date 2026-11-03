import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { resolve } from 'node:path';

// Two independent entry points: the capture overlay and a pin window. Neither
// loads the other, and there is no main window.
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  build: {
    rollupOptions: {
      input: {
        overlay: resolve(process.cwd(), 'screen-capture.html'),
        pin: resolve(process.cwd(), 'screenshot-pin.html'),
      },
    },
  },
});
