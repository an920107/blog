<script lang="ts">
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
	import PostOverallPage from '$lib/post/framework/ui/PostOverallPage.svelte';
	import { Container } from '$lib/container';
	import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';
	import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/labelsListedStore';

	const { data }: PageProps = $props();
	const container = getContext<Container>(Container.name);

	const keyword = $derived(data.searchParams.keyword);
	const labelId = $derived(data.searchParams.labelId);

	const initialData = {
		posts: data.dehydratedData.posts?.map((post) => PostInfoViewModel.rehydrate(post)),
		labels: data.dehydratedData.labels?.map((label) => LabelViewModel.rehydrate(label)),
	};
	const postListedStore = container.createPostsListedStore(initialData.posts);
	const labelListedStore = container.createLabelsListedStore(initialData.labels);
	setContext(PostsListedStore.name, postListedStore);
	setContext(LabelsListedStore.name, labelListedStore);

	$effect(() => {
		const posts = data.dehydratedData.posts?.map((post) => PostInfoViewModel.rehydrate(post));
		if (posts) {
			postListedStore.setData(posts);
		}
	});
</script>

<PostOverallPage {keyword} {labelId} />
