import { error } from '@sveltejs/kit';

import { buildFeedDocument } from '$lib/seo/framework/xml/feedDocument';
import { Strings } from '$lib/strings';

import type { RequestHandler } from './$types';

// The document is derived from database state, so it is generated on demand and cached
// briefly rather than baked into the image at build time.
const CACHE_CONTROL = 'public, max-age=300';

export const GET: RequestHandler = async ({ locals }) => {
	const store = locals.container.createPostPagesListedStore();
	const state = await store.trigger();

	const postPages = state.data;
	if (state.isError() || postPages === null) {
		error(500, Strings.FAILED_TO_GENERATE_FEED);
	}

	const body = buildFeedDocument(postPages);

	return new Response(body, {
		headers: {
			'Content-Type': 'application/rss+xml; charset=utf-8',
			'Cache-Control': CACHE_CONTROL,
		},
	});
};
