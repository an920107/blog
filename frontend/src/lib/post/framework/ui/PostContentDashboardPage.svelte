<script lang="ts">
	import { getContext, onMount } from 'svelte';
	import { toast } from 'svelte-sonner';

	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { PostLoadedStore } from '$lib/post/adapter/presenter/store/postLoadedStore';
	import { PostUpdatedStore } from '$lib/post/adapter/presenter/store/postUpdatedStore';
	import EditPostDialog, {
		type EditPostDialogFormParams,
	} from '$lib/post/framework/ui/EditPostDialog.svelte';
	import MarkdownRenderer from '$lib/post/framework/ui/MarkdownRenderer.svelte';
	import { HoverCard } from '$lib/shared/framework/components/ui/hover-card';
	import HoverCardContent from '$lib/shared/framework/components/ui/hover-card/hover-card-content.svelte';
	import HoverCardTrigger from '$lib/shared/framework/components/ui/hover-card/hover-card-trigger.svelte';
	import Table from '$lib/shared/framework/components/ui/table/table.svelte';
	import TableBody from '$lib/shared/framework/components/ui/table/table-body.svelte';
	import TableCell from '$lib/shared/framework/components/ui/table/table-cell.svelte';
	import TableHead from '$lib/shared/framework/components/ui/table/table-head.svelte';
	import TableRow from '$lib/shared/framework/components/ui/table/table-row.svelte';
	import { Strings } from '$lib/strings';

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
			publishedTime: postLoadedState.data.info.publishedTime?.nativeDate ?? null,
		};
	});

	async function onSubmit(params: EditPostDialogFormParams): Promise<boolean> {
		if (!post) {
			toast.error(Strings.POST_NOT_FOUND);
			return false;
		}

		const state = await updatePost({
			id: post.id,
			params: {
				...params,
				publishedTime: params.publishedTime,
			},
		});

		if (state.isSuccess()) {
			loadPost(post.id);
			toast.success(Strings.POST_UPDATED);
		} else if (state.isError()) {
			toast.error(Strings.FAILED_TO_UPDATE_POST, {
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
		<h1 class="py-16 text-5xl font-bold text-gray-800">{Strings.POST_DETAILS}</h1>
		{#key formDefaultValues}
			<EditPostDialog
				mode="update"
				triggerButtonText={Strings.EDIT}
				disabled={!postLoadedState.isSuccess() || postUpdatedState.isLoading()}
				defaultValues={formDefaultValues ?? undefined}
				{onSubmit}
			/>
		{/key}
	</div>
	<Table>
		<TableBody>
			<TableRow>
				<TableHead>{Strings.ID}</TableHead>
				<TableCell>{post?.id}</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>{Strings.SEMANTIC_ID}</TableHead>
				<TableCell>
					<span class="text-wrap">{post?.info.semanticId}</span>
				</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>{Strings.TITLE}</TableHead>
				<TableCell>
					<span class="text-wrap">{post?.info.title}</span>
				</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>{Strings.LABELS}</TableHead>
				<TableCell>
					<div class="flex flex-wrap gap-2">
						{#each post?.info.labels ?? [] as label (label.id)}
							<PostLabel {label} />
						{/each}
					</div>
				</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>{Strings.DESCRIPTION}</TableHead>
				<TableCell>
					<span class="text-wrap">{post?.info.description}</span>
				</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>{Strings.PREVIEW_IMAGE_URL}</TableHead>
				<TableCell>
					{#if post?.info.previewImageUrl}
						{@const url = post.info.previewImageUrl}
						<HoverCard>
							<HoverCardTrigger href={url} target="_blank" class="text-wrap underline">
								{post?.info.previewImageUrl ?? Strings.NONE}
							</HoverCardTrigger>
							<HoverCardContent class="aspect-video w-96">
								<img src={url} alt={Strings.PREVIEW} class="h-full w-full object-cover" />
							</HoverCardContent>
						</HoverCard>
					{:else}
						<span>{Strings.NONE}</span>
					{/if}
				</TableCell>
			</TableRow>
			<TableRow>
				<TableHead>{Strings.PUBLISHED_TIME}</TableHead>
				<TableCell
					>{post?.info.publishedTime?.toLocalISOString() ?? Strings.NOT_PUBLISHED}</TableCell
				>
			</TableRow>
			<TableRow>
				<TableHead>{Strings.CONTENT_LENGTH}</TableHead>
				<TableCell>{post?.content.length ?? 0}</TableCell>
			</TableRow>
		</TableBody>
	</Table>

	<div class="mt-16 max-w-3xl">
		<MarkdownRenderer content={post?.content ?? ''} />
	</div>
</div>
