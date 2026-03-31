import { Environment } from '$lib/environment';

export abstract class Links {
	static readonly YOUTUBE: string = 'https://www.youtube.com/@squidspirit16';
	static readonly EMAIL: string = 'mailto:squid@squidspirit.com';
	static readonly RSS: string = Environment.APP_BASE_URL + 'rss.xml';
	static readonly SOURCE_CODE: string = 'https://git.squidspirit.com/squid/blog';
	static readonly PRIVACY_POLICY: string = '/terms/privacy-policy';
	static readonly AI_USAGE_POLICY: string = '/terms/ai-usage-policy';
}
