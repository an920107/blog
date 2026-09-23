import { Feed, type Item } from 'feed';

import { Environment } from '$lib/environment';
import { Links } from '$lib/links';
import type { PostPage } from '$lib/seo/domain/entity/postPage';
import { Strings } from '$lib/strings';

const FEED_LANGUAGE = 'zh-TW';

/**
 * How many of the most recent posts the feed advertises. A feed is a rolling window over
 * the newest content, unlike the sitemap which lists every page.
 */
const FEED_ITEM_LIMIT = 20;

/**
 * Builds feed.xml from the published posts. Server-only: it must never be imported from
 * client code.
 */
export function buildFeedDocument(postPages: readonly PostPage[]): string {
	const feed = new Feed({
		title: Strings.APP_NAME,
		id: Environment.APP_BASE_URL,
		link: Environment.APP_BASE_URL,
		description: Strings.SITE_DESCRIPTION,
		language: FEED_LANGUAGE,
		generator: false,
		feed: Links.RSS.href,
		updated: new Date(),
	});

	const latestPostPages = [...postPages]
		.sort((a, b) => b.publishedTime.getTime() - a.publishedTime.getTime())
		.slice(0, FEED_ITEM_LIMIT);

	for (const page of latestPostPages) {
		const item: Item = {
			title: page.title,
			// The guid is the post's stable semantic id rather than its URL, so changing the URL
			// scheme does not make every post reappear as unread for existing subscribers. The
			// package renders it as <guid isPermaLink="false">.
			guid: page.semanticId,
			link: page.url.href,
			description: page.description,
			date: page.lastModified,
			published: page.publishedTime,
			category: page.categories.map((name) => ({ name })),
		};

		// RSS enclosures require a MIME type and a length, both of which the backend
		// provides for uploaded preview images.
		if (
			page.previewImageUrl &&
			page.previewImageMimeType !== null &&
			page.previewImageSize !== null
		) {
			item.image = {
				url: page.previewImageUrl.href,
				type: page.previewImageMimeType,
				length: page.previewImageSize,
			};
		}

		feed.addItem(item);
	}

	return feed.rss2();
}
