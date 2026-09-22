import { parsePostListSearchParams } from '$lib/post/framework/ui/postListSearchParams';

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, url }) => {
	const { container } = locals;

	const postsListedStore = container.createPostsListedStore();
	const { trigger: loadPosts } = postsListedStore;

	const searchParams = parsePostListSearchParams(url.searchParams);

	const postsListedState = await loadPosts({ showUnpublished: false, ...searchParams });

	return {
		dehydratedData: {
			posts: postsListedState.data?.map((post) => post.dehydrate()),
		},
		searchParams,
	};
};
