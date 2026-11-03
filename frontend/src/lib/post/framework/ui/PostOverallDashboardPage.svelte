<script lang="ts">
	import { getContext, onMount } from 'svelte';
	import { toast } from 'svelte-sonner';

	import { resolve } from '$app/paths';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { PostCreatedStore } from '$lib/post/adapter/presenter/store/postCreatedStore';
	import { PostsListedStore } from '$lib/post/adapter/presenter/store/postsListedStore';
	import EditPostDialog, {
		type EditPostDialogFormParams,
	} from '$lib/post/framework/ui/EditPostDialog.svelte';
	import Table from '$lib/shared/framework/components/ui/table/table.svelte';
	import TableBody from '$lib/shared/framework/components/ui/table/table-body.svelte';
	import TableCell from '$lib/shared/framework/components/ui/table/table-cell.svelte';
	import TableHead from '$lib/shared/framework/components/ui/table/table-head.svelte';
	import TableHeader from '$lib/shared/framework/components/ui/table/table-header.svelte';
	import TableRow from '$lib/shared/framework/components/ui/table/table-row.svelte';
	import { Strings } from '$lib/strings';

	const postCreatedStore = getContext<PostCreatedStore>(PostCreatedStore.name);
	const postCreatedState = $derived($postCreatedStore);
	const { trigger: createPost } = postCreatedStore;

	const postsListedStore = getContext<PostsListedStore>(PostsListedStore.name);
	const postsListedState = $derived($postsListedStore);
	const { trigger: loadPosts } = postsListedStore;

	async function onCreatePostDialogSubmit(params: EditPostDialogFormParams): Promise<boolean> {
		const state = await createPost({
			semanticId: params.semanticId,
			title: params.title,
		});

		if (state.isSuccess()) {
			loadPosts({ showUnpublished: true });
			toast.success(`${Strings.POST_CREATED_PREFIX} ${state.data.id}`);
		} else if (state.isError()) {
			toast.error(Strings.FAILED_TO_CREATE_POST, {
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
		<h1 class="py-16 text-5xl font-bold text-gray-800">{Strings.DASHBOARD_POST}</h1>
		<EditPostDialog
			mode="create"
			triggerButtonText={Strings.CREATE}
			disabled={postCreatedState.isLoading()}
			onSubmit={onCreatePostDialogSubmit}
		/>
	</div>
	<Table>
		<TableHeader>
			<TableRow>
				<TableHead>{Strings.ID}</TableHead>
				<TableHead>{Strings.SEMANTIC_ID}</TableHead>
				<TableHead>{Strings.TITLE}</TableHead>
				<TableHead>{Strings.LABELS}</TableHead>
				<TableHead>{Strings.PUBLISHED_TIME}</TableHead>
			</TableRow>
		</TableHeader>
		<TableBody>
			{#each postsListedState.data ?? [] as postInfo (postInfo.id)}
				<TableRow>
					<TableHead>
						<a
							href={resolve('/dashboard/post/[id]', { id: postInfo.id.toString() })}
							class="underline">{postInfo.id}</a
						>
					</TableHead>
					<TableCell>
						<span class="text-wrap">{postInfo.semanticId}</span>
					</TableCell>
					<TableCell>
						<span class="text-wrap">{postInfo.title}</span>
					</TableCell>
					<TableCell>
						<div class="flex flex-row flex-wrap gap-2">
							{#each postInfo.labels as label (label.id)}
								<PostLabel {label} />
							{/each}
						</div>
					</TableCell>
					<TableCell>
						{postInfo.publishedTime?.toLocalISOString() ?? Strings.NOT_PUBLISHED}
					</TableCell>
				</TableRow>
			{/each}
		</TableBody>
	</Table>
</div>
