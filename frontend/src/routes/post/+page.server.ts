import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, url }) => {
	const { container } = locals;

	const postsListedStore = container.createPostsListedStore();
	const { trigger: loadPosts } = postsListedStore;
	const labelsListedStore = container.createLabelsListedStore();
	const { trigger: listLabels } = labelsListedStore;

	const keyword = url.searchParams.get('keyword') ?? undefined;
	const labelIdParam = url.searchParams.get('label_id');
	const labelId = labelIdParam ? Number(labelIdParam) : undefined;

	const postsListedState = await loadPosts({ showUnpublished: false, keyword, labelId });
	const labelsListedState = await listLabels();

	return {
		dehydratedData: {
			posts: postsListedState.data?.map((post) => post.dehydrate()),
			labels: labelsListedState.data?.map((label) => label.dehydrate()),
		},
		searchParams: { keyword, labelId },
	};
};
