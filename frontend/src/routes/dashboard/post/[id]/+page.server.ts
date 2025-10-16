import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, params }) => {
	const { container } = locals;

	const store = container.createPostLoadedStore();
	const { trigger: loadPost } = store;

	const numericId = Number(params.id);
	if (isNaN(numericId)) {
		error(400, { message: 'Invalid post ID' });
	}

	const state = await loadPost(numericId);
	if (!state.data) {
		error(404, { message: 'Post not found' });
	}

	return {
		id: numericId,
		dehydratedData: state.data.dehydrate(),
	};
};
