import postcss from './postcss.config.js';
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// https://vitejs.dev/config/
export default defineConfig({
  clearScreen: false,
  plugins: [svelte()],
  server: { port: 5050 },
  css: {
	  postcss
  }
});
