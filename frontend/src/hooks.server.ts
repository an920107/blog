import * as Sentry from '@sentry/sveltekit';
import type { Handle } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';

import { Container } from '$lib/container';
import { Environment } from '$lib/environment';

Sentry.init({
	dsn: Environment.SENTRY_DSN,
	tracesSampleRate: 1,
	enableLogs: true,
	release: App.__VERSION__,
	environment: process.env.NODE_ENV || 'development',
});

export const handle: Handle = sequence(Sentry.sentryHandle(), ({ event, resolve }) => {
	event.locals.container ??= new Container(event.fetch);

	return resolve(event);
});

export const handleError = Sentry.handleErrorWithSentry();
