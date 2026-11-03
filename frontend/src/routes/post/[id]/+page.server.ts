import { error } from '@sveltejs/kit';

import { Strings } from '$lib/strings';

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, params }) => {
	const { container } = locals;

	const store = container.createPostLoadedStore();
	const { trigger: loadPost } = store;

	const state = await loadPost(params.id);
	if (!state.data) {
		error(404, { message: Strings.POST_NOT_FOUND });
	}

	return {
		id: params.id,
		dehydratedData: state.data.dehydrate(),
	};
};
