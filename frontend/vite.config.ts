import { sentrySvelteKit } from '@sentry/sveltekit';
import { sentryVitePlugin } from '@sentry/vite-plugin';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig, loadEnv } from 'vite';

// eslint-disable-next-line no-restricted-imports
import { version } from './package.json';

export default defineConfig(({ mode }) => {
	const env = loadEnv(mode, process.cwd(), '');
	const sentryProjects = ['blog-frontend', 'blog-beta-frontend'];

	return {
		plugins: [
			sentrySvelteKit(),
			...sentryProjects.map((project) =>
				sentryVitePlugin({
					org: 'squidspirit',
					project,
					authToken: env.SENTRY_AUTH_TOKEN,
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
	};
});
