import { error } from '@sveltejs/kit';

import { TERMS_DOCUMENTS } from '$lib/terms/framework/termsDocuments';

import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
	const { name } = params;
	if (!TERMS_DOCUMENTS.map((doc) => doc.pathname).includes(name)) {
		error(404, { message: `Terms document "${params.name}" not found` });
	}

	const path = `/terms/${encodeURIComponent(name)}.md`;
	const response = await fetch(path);

	if (!response.ok) {
		error(response.status, { message: `Failed to load terms document "${params.name}"` });
	}

	return {
		title: TERMS_DOCUMENTS.find((doc) => doc.pathname === name)?.title,
		content: await response.text(),
	};
};
