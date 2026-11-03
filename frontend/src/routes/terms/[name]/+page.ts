import { error } from '@sveltejs/kit';

import { Strings } from '$lib/strings';
import { TERMS_DOCUMENTS } from '$lib/terms/framework/termsDocuments';

import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
	const { name } = params;
	if (!TERMS_DOCUMENTS.map((doc) => doc.pathname).includes(name)) {
		error(404, {
			message: `${Strings.TERMS_NOT_FOUND_PREFIX} "${params.name}" ${Strings.TERMS_NOT_FOUND_SUFFIX}`,
		});
	}

	const path = `/terms/${encodeURIComponent(name)}.md`;
	const response = await fetch(path);

	if (!response.ok) {
		error(response.status, {
			message: `${Strings.FAILED_TO_LOAD_TERMS} "${params.name}"`,
		});
	}

	return {
		title: TERMS_DOCUMENTS.find((doc) => doc.pathname === name)?.title,
		content: await response.text(),
	};
};
