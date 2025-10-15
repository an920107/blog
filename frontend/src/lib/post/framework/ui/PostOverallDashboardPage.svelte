<script lang="ts">
	import TableBody from '$lib/common/framework/components/ui/table/table-body.svelte';
	import TableCell from '$lib/common/framework/components/ui/table/table-cell.svelte';
	import TableHead from '$lib/common/framework/components/ui/table/table-head.svelte';
	import TableHeader from '$lib/common/framework/components/ui/table/table-header.svelte';
	import TableRow from '$lib/common/framework/components/ui/table/table-row.svelte';
	import Table from '$lib/common/framework/components/ui/table/table.svelte';
	import { PostCreatedStore } from '$lib/post/adapter/presenter/postCreatedStore';
	import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';
	import CreatePostDialog, {
		type CreatePostDialogFormParams,
	} from '$lib/post/framework/ui/CreatePostDialog.svelte';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { getContext, onMount } from 'svelte';
	import { toast } from 'svelte-sonner';

	const postCreatedStore = getContext<PostCreatedStore>(PostCreatedStore.name);
	const postCreatedState = $derived($postCreatedStore);
	const { trigger: createPost } = postCreatedStore;

	const postsListedStore = getContext<PostsListedStore>(PostsListedStore.name);
	const postsListedState = $derived($postsListedStore);
	const { trigger: loadPosts } = postsListedStore;

	async function onCreatePostDialogSubmit(params: CreatePostDialogFormParams): Promise<boolean> {
		const state = await createPost(params);

		if (state.isSuccess()) {
			loadPosts({ showUnpublished: true });
			toast.success(`Post created successfully with ID: ${state.data.id}`);
		} else if (state.isError()) {
			toast.error('Failed to create post', {
				description: state.error.message,
			});
			return false;
		}

		return true;
	}

	onMount(() => loadPosts({ showUnpublished: true }));
</script>

<div class="dashboard-container mb-10">
	<div class="flex flex-row items-center justify-between">
		<h1 class="py-16 text-5xl font-bold text-gray-800">Post</h1>
		<CreatePostDialog disabled={postCreatedState.isLoading()} onSubmit={onCreatePostDialogSubmit} />
	</div>
	<Table>
		<TableHeader>
			<TableRow>
				<TableHead>ID</TableHead>
				<TableHead>Title</TableHead>
				<TableHead>Labels</TableHead>
				<TableHead>Published Time</TableHead>
			</TableRow>
		</TableHeader>
		<TableBody>
			{#each postsListedState.data ?? [] as postInfo (postInfo.id)}
				<TableRow>
					<TableCell>{postInfo.id}</TableCell>
					<TableCell><span class="text-wrap">{postInfo.title}</span></TableCell>
					<TableCell>
						<div class="flex flex-row flex-wrap gap-2">
							{#each postInfo.labels as label (label.id)}
								<PostLabel {label} />
							{/each}
						</div>
					</TableCell>
					<TableCell>{postInfo.formattedPublishedTime}</TableCell>
				</TableRow>
			{/each}
		</TableBody>
	</Table>
</div>
