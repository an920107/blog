import { sentrySvelteKit } from '@sentry/sveltekit';
import { sentryVitePlugin } from '@sentry/vite-plugin';
import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

import { version } from './package.json';

const sentryProjects = ['blog-frontend', 'blog-beta-frontend'];

export default defineConfig({
	plugins: [
		sentrySvelteKit(),
		sentryProjects.map((project) =>
			sentryVitePlugin({
				org: 'squidspirit',
				project,
				release: {
					name: version,
				},
			})
		),
		tailwindcss(),
		sveltekit(),
	],
	define: {
		'App.__VERSION__': JSON.stringify(version),
	},
	server: {
		proxy: {
			'/api': {
				target: 'http://127.0.0.1:8080',
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, ''),
			},
		},
	},
});
