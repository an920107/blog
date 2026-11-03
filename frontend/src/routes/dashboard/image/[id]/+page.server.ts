import { error } from '@sveltejs/kit';

import { Strings } from '$lib/strings';

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, params }) => {
	const { container } = locals;
	const id = Number(params.id);

	if (isNaN(id)) {
		throw error(400, Strings.INVALID_IMAGE_ID);
	}

	const store = container.createImageLoadedStore();
	const { trigger: loadImage } = store;

	const state = await loadImage(id);
	if (state.isError()) {
		throw error(404, Strings.IMAGE_NOT_FOUND);
	}

	return {
		dehydratedData: state.data?.dehydrate(),
	};
};
