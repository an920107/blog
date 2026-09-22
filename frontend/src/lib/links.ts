import { Environment } from '$lib/environment';

export abstract class Links {
	static readonly YOUTUBE: URL = new URL('https://www.youtube.com/@squidspirit16');
	static readonly EMAIL: URL = new URL('mailto:squid@squidspirit.com');
	static readonly RSS: URL = new URL('feed.xml', Environment.APP_BASE_URL);
	static readonly SOURCE_CODE: URL = new URL('https://git.squidspirit.com/squid/blog');

	static readonly EMAIL_SHARE: (target: URL, subject: string, body: string) => URL = (
		target: URL,
		subject: string,
		body: string
	) => {
		const url = new URL('mailto:');
		// A mailto: query cannot be built with URL + searchParams: URLSearchParams encodes
		// spaces as '+', but RFC 6068 requires '%20' and mail clients do not restore '+'
		// back to a space. Encode the values manually instead.
		url.search = `subject=${encodeURIComponent(subject)}&body=${encodeURIComponent(body)}`;
		return url;
	};

	static readonly FACEBOOK_SHARE: (target: URL) => URL = (target: URL) => {
		const url = new URL('https://www.facebook.com/sharer/sharer.php');
		url.searchParams.append('u', target.href);
		return url;
	};

	static readonly LINKEDIN_SHARE: (target: URL) => URL = (target: URL) => {
		const url = new URL('https://www.linkedin.com/sharing/share-offsite/');
		url.searchParams.append('url', target.href);
		return url;
	};

	static readonly X_SHARE: (target: URL, text: string) => URL = (target: URL, text: string) => {
		const url = new URL('https://x.com/intent/post');
		url.searchParams.append('url', target.href);
		url.searchParams.append('text', text);
		return url;
	};
}
