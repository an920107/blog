<script lang="ts">
	import { getContext, onMount } from 'svelte';
	import { toast } from 'svelte-sonner';

	import { resolve } from '$app/paths';
	import { LabelLoadedStore } from '$lib/label/adapter/presenter/store/labelLoadedStore';
	import { LabelUpdatedStore } from '$lib/label/adapter/presenter/store/labelUpdatedStore';
	import { ColorViewModel } from '$lib/label/adapter/presenter/viewModel/colorViewModel';
	import ColorCode from '$lib/label/framework/ui/ColorCode.svelte';
	import EditLabelDialog, {
		type EditLabelDialogFormParams,
	} from '$lib/label/framework/ui/EditLabelDialog.svelte';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { PostsListedStore } from '$lib/post/adapter/presenter/store/postsListedStore';
	import Table from '$lib/shared/framework/components/ui/table/table.svelte';
	import TableBody from '$lib/shared/framework/components/ui/table/table-body.svelte';
	import TableCell from '$lib/shared/framework/components/ui/table/table-cell.svelte';
	import TableRow from '$lib/shared/framework/components/ui/table/table-row.svelte';
	import { Strings } from '$lib/strings';

	const { id }: { id: number } = $props();

	const labelLoadedStore = getContext<LabelLoadedStore>(LabelLoadedStore.name);
	const labelLoadedState = $derived($labelLoadedStore);
	const { trigger: loadLabel } = labelLoadedStore;
	const label = $derived(labelLoadedState.data);

	const labelUpdatedStore = getContext<LabelUpdatedStore>(LabelUpdatedStore.name);
	const labelUpdatedState = $derived($labelUpdatedStore);
	const { trigger: updateLabel } = labelUpdatedStore;

	const relatedPostsStore = getContext<PostsListedStore>(PostsListedStore.name);
	const relatedPostsState = $derived($relatedPostsStore);
	const { trigger: loadRelatedPosts } = relatedPostsStore;

	const formDefaultValues: EditLabelDialogFormParams | null = $derived.by(() => {
		if (labelLoadedState.data === null) {
			return null;
		}
		return {
			name: labelLoadedState.data.name,
			color: labelLoadedState.data.color.hexWithoutAlpha,
		};
	});

	async function onSubmit(params: EditLabelDialogFormParams): Promise<boolean> {
		if (!label) {
			toast.error(Strings.LABEL_NOT_FOUND);
			return false;
		}

		const colorViewModel = ColorViewModel.fromHex(params.color);
		const color = colorViewModel.toEntity();

		const state = await updateLabel({
			id: label.id,
			params: {
				name: params.name,
				color: color,
			},
		});

		if (state.isSuccess()) {
			loadLabel(label.id);
			toast.success(Strings.LABEL_UPDATED);
		} else if (state.isError()) {
			toast.error(Strings.FAILED_TO_UPDATE_LABEL, {
				description: state.error.message,
			});
			return false;
		}

		return true;
	}

	onMount(() => {
		loadLabel(id);
		loadRelatedPosts({ showUnpublished: true, labelId: id });
	});
</script>

<div class="dashboard-container mb-10">
	<div class="flex flex-row items-center justify-between">
		<h1 class="py-16 text-5xl font-bold text-gray-800">{Strings.LABEL_DETAILS}</h1>
		{#key formDefaultValues}
			<EditLabelDialog
				title={Strings.UPDATE_LABEL}
				triggerButtonText={Strings.EDIT}
				disabled={!labelLoadedState.isSuccess() || labelUpdatedState.isLoading()}
				defaultValues={formDefaultValues ?? undefined}
				{onSubmit}
			/>
		{/key}
	</div>
	<Table>
		<TableBody>
			<TableRow>
				<TableCell class="font-medium">{Strings.ID}</TableCell>
				<TableCell>{label?.id}</TableCell>
			</TableRow>
			<TableRow>
				<TableCell class="font-medium">{Strings.NAME}</TableCell>
				<TableCell>{label?.name}</TableCell>
			</TableRow>
			<TableRow>
				<TableCell class="font-medium">{Strings.COLOR}</TableCell>
				<TableCell>
					{#if label}
						<ColorCode color={label.color} />
					{/if}
				</TableCell>
			</TableRow>
			<TableRow>
				<TableCell class="font-medium">{Strings.PREVIEW}</TableCell>
				<TableCell>
					{#if label}
						<PostLabel {label} />
					{/if}
				</TableCell>
			</TableRow>
			<TableRow>
				<TableCell class="font-medium">{Strings.RELATED_POSTS}</TableCell>
				<TableCell>
					{#each relatedPostsState.data ?? [] as postInfo (postInfo.id)}
						<p>
							<a
								href={resolve('/dashboard/post/[id]', { id: postInfo.id.toString() })}
								class="underline"
							>
								{postInfo.title}
							</a>
						</p>
					{/each}
				</TableCell>
			</TableRow>
		</TableBody>
	</Table>
</div>
