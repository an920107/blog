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

	return resolve(event);
});

export const handleError = Sentry.handleErrorWithSentry();
