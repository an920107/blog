import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, params }) => {
	const { container } = locals;
	const id = Number(params.id);

	if (isNaN(id)) {
		throw error(400, 'Invalid image ID');
	}

	const store = container.createImageLoadedStore();
	const { trigger: loadImage } = store;

	const state = await loadImage(id);
	if (state.isError()) {
		throw error(404, 'Image not found');
	}

	return {
		dehydratedData: state.data?.dehydrate(),
	};
};
