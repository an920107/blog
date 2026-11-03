import { error } from '@sveltejs/kit';

import { Strings } from '$lib/strings';

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, params }) => {
	const { container } = locals;

	const store = container.createPostLoadedStore();
	const { trigger: loadPost } = store;

	const numericId = Number(params.id);
	if (isNaN(numericId)) {
		error(400, { message: Strings.INVALID_POST_ID });
	}

	const state = await loadPost(numericId);
	if (!state.data) {
		error(404, { message: Strings.POST_NOT_FOUND });
	}

	return {
		id: numericId,
		dehydratedData: state.data.dehydrate(),
	};
};
