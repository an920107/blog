import { sequence } from '@sveltejs/kit/hooks';
import * as Sentry from '@sentry/sveltekit';
import type { Handle } from '@sveltejs/kit';
import { Environment } from '$lib/environment';
import { Container } from '$lib/container';

Sentry.init({
	dsn: Environment.SENTRY_DSN,
	tracesSampleRate: 1,
	enableLogs: true,
});

export const handle: Handle = sequence(Sentry.sentryHandle(), ({ event, resolve }) => {
	event.locals.container ??= new Container(event.fetch);

	const userAgent = event.request.headers.get('user-agent') || '';
	const isGoogleBot = userAgent.toLowerCase().includes('googlebot');

	return resolve(event, {
		transformPageChunk: ({ html }) => {
			if (isGoogleBot) {
				return html.replace(/<link\s+rel="modulepreload"\s+as="script".+\/>/g, '');
			}
			return html;
		},
	});
});

export const handleError = Sentry.handleErrorWithSentry();
