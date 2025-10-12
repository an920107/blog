import { PostListEventType } from '$lib/post/adapter/presenter/postListBloc';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals }) => {
	const { postListBloc } = locals;

	const state = await postListBloc.dispatch({ event: PostListEventType.PostListLoadedEvent });

	return {
		dehydratedData: state.data?.map((post) => post.dehydrate()),
	};
};
