import { env } from '$env/dynamic/public';

export abstract class Environment {
	static readonly APP_BASE_URL = env.PUBLIC_APP_BASE_URL ?? 'http://localhost:5173/';
	static readonly API_BASE_URL = env.PUBLIC_API_BASE_URL ?? 'http://localhost:5173/api/';
	static readonly ADSENSE_CLIENT_ID = env.PUBLIC_ADSENSE_CLIENT_ID ?? '';
	static readonly GA_MEASUREMENT_ID = env.PUBLIC_GA_MEASUREMENT_ID ?? '';
	static readonly SENTRY_DSN = env.PUBLIC_SENTRY_DSN ?? '';
}
