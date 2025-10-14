import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, params }) => {
	const { container } = locals;
	const store = container.createPostLoadedStore();
	const { trigger: loadPost } = store;

	const state = await loadPost(params.id);
	if (!state.data) {
		error(404, { message: 'Post not found' });
	}

	return {
		dehydratedData: state.data.dehydrate(),
	};
};
