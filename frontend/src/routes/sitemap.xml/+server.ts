import { error } from '@sveltejs/kit';

import { buildSitemapDocument } from '$lib/seo/framework/xml/sitemapDocument';

import type { RequestHandler } from './$types';

// The document is derived from database state, so it is generated on demand and cached
// briefly rather than baked into the image at build time.
const CACHE_CONTROL = 'public, max-age=300';

export const GET: RequestHandler = async ({ locals }) => {
	const store = locals.container.createPostPagesListedStore();
	const state = await store.trigger();

	const postPages = state.data;
	if (state.isError() || postPages === null) {
		error(500, 'Failed to generate the sitemap');
	}

	const body = await buildSitemapDocument(postPages);

	return new Response(body, {
		headers: {
			'Content-Type': 'application/xml; charset=utf-8',
			'Cache-Control': CACHE_CONTROL,
		},
	});
};
