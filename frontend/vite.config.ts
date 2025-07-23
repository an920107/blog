import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

import { version } from './package.json';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	define: {
		'App.__VERSION__': JSON.stringify(version)
	},
	server: {
		proxy: {
			'/api': {
				target: 'http://127.0.0.1:8080',
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, '')
			}
		}
	}
});
