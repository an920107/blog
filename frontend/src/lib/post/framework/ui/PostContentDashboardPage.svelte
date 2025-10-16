<script lang="ts">
	import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
	import { PostUpdatedStore } from '$lib/post/adapter/presenter/postUpdatedStore';
	import EditPostDialog, {
		type EditPostDialogFormParams,
	} from '$lib/post/framework/ui/EditPostDialog.svelte';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { getContext, onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import Table from '$lib/common/framework/components/ui/table/table.svelte';
	import TableBody from '$lib/common/framework/components/ui/table/table-body.svelte';
	import TableRow from '$lib/common/framework/components/ui/table/table-row.svelte';
	import TableCell from '$lib/common/framework/components/ui/table/table-cell.svelte';
	import TableHead from '$lib/common/framework/components/ui/table/table-head.svelte';
	import MardownRenderer from '$lib/post/framework/ui/MardownRenderer.svelte';

	const { id }: { id: number } = $props();

	const postLoadedStore = getContext<PostLoadedStore>(PostLoadedStore.name);
	const postLoadedState = $derived($postLoadedStore);
	const { trigger: loadPost } = postLoadedStore;
	const post = $derived(postLoadedState.data);

	const postUpdatedStore = getContext<PostUpdatedStore>(PostUpdatedStore.name);
	const postUpdatedState = $derived($postUpdatedStore);
	const { trigger: updatePost } = postUpdatedStore;

	const formDefaultValues: EditPostDialogFormParams | null = $derived.by(() => {
		if (postLoadedState.data === null) {
			return null;
		}
		return {
			semanticId: postLoadedState.data.info.semanticId,
			title: postLoadedState.data.info.title,
			description: postLoadedState.data.info.description,
			content: postLoadedState.data.content,
			labelIds: postLoadedState.data.info.labels.map((label) => label.id),
			previewImageUrl: postLoadedState.data.info.previewImageUrl,
			publishedTime: postLoadedState.data.info.publishedTime,
		};
	});

	async function onSubmit(params: EditPostDialogFormParams): Promise<boolean> {
		if (!post) {
			toast.error('Post not found');
			return false;
		}

		const state = await updatePost({ id: post.id, params });

		if (state.isSuccess()) {
			loadPost(post.id);
			toast.success('Post updated successfully');
		} else if (state.isError()) {
			toast.error('Failed to update post', {
				description: state.error.message,
			});
			return false;
		}

		return true;
	}

	onMount(() => {
		loadPost(id);
	});
</script>

<div class="dashboard-container mb-10">
	<div class="flex flex-row items-center justify-between">
		<h1 class="py-16 text-5xl font-bold text-gray-800">Post Details</h1>
		{#key formDefaultValues}
			<EditPostDialog
				mode="update"
				triggerButtonText="Edit"
				disabled={!postLoadedState.isSuccess() || postUpdatedState.isLoading()}
				defaultValues={formDefaultValues ?? undefined}
				{onSubmit}
			/>
		{/key}
	</div>
	<Table>
		<TableBody>
			<TableRow>
				<TableHead>ID</TableHead>
				<TableCell>{post?.id}</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>Semantic ID</TableHead>
				<TableCell>
					<span class="text-wrap">{post?.info.semanticId}</span>
				</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>Title</TableHead>
				<TableCell>
					<span class="text-wrap">{post?.info.title}</span>
				</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>Labels</TableHead>
				<TableCell>
					<div class="flex flex-wrap gap-2">
						{#each post?.info.labels ?? [] as label (label.id)}
							<PostLabel {label} />
						{/each}
					</div>
				</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>Description</TableHead>
				<TableCell>
					<span class="text-wrap">{post?.info.description}</span>
				</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>Preview Image URL</TableHead>
				<TableCell>
					<a
						href={post?.info.previewImageUrl?.toString()}
						target="_blank"
						class="text-wrap underline"
					>
						{post?.info.previewImageUrl?.toString() ?? 'None'}
					</a>
				</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>Published Time</TableHead>
				<TableCell>{post?.info.publishedTime?.toLocalISOString() ?? 'Not Published'}</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>Content Length</TableHead>
				<TableCell>{post?.content.length ?? 0}</TableCell>
			</TableRow>
		</TableBody>
	</Table>

	<div class="mt-16 max-w-3xl">
		<MardownRenderer content={post?.content ?? ''} />
	</div>
</div>
