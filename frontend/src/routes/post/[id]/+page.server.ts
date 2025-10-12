import { PostEventType } from '$lib/post/adapter/presenter/postBloc';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, params }) => {
	const { postBloc } = locals;

	const state = await postBloc.dispatch({
		event: PostEventType.PostLoadedEvent,
		id: params.id,
	});
	if (!state.data) {
		error(404, { message: 'Post not found' });
	}

	return {
		dehydratedData: state.data.dehydrate(),
	};
};
