import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

const TERMS_DOCUMENTS = ['privacy-policy', 'ai-usage-policy'];

export const load: PageLoad = async ({ fetch, params }) => {
	const { name } = params;
	if (!TERMS_DOCUMENTS.includes(name)) {
		error(404, { message: `Terms document "${params.name}" not found` });
	}

	const path = `/terms/${encodeURIComponent(name)}.md`;
	const response = await fetch(path);

	if (!response.ok) {
		error(response.status, { message: `Failed to load terms document "${params.name}"` });
	}

	return {
		content: await response.text(),
	};
};
