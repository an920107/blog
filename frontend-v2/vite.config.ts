import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

import { version } from './package.json';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	define: {
		'App.__VERSION__': JSON.stringify(version)
	}
});
