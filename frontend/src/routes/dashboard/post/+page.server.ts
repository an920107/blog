import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals }) => {
	const { container } = locals;
	const store = container.createPostsListedStore();
	const { trigger: loadPosts } = store;

	const state = await loadPosts();

	return {
		dehydratedData: state.data?.map((post) => post.dehydrate()),
	};
};
