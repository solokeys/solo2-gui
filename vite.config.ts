import postcss from './postcss.config.js';
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { resolve } from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  clearScreen: false,
  css: {
	  postcss
  },
  // https://mtm.dev/svelte-vite-aliases
  resolve: {
	alias: {
	  $lib: resolve('./src/lib'),
	  $components: resolve('./src/components'),
	  $routes: resolve('./src/routes'),
	}
  },
  plugins: [svelte()],
  server: { port: 5050 }
});
