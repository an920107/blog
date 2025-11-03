<script lang="ts">
	import { LabelLoadedStore } from '$lib/label/adapter/presenter/labelLoadedStore';
	import { LabelUpdatedStore } from '$lib/label/adapter/presenter/labelUpdatedStore';
	import { ColorViewModel } from '$lib/label/adapter/presenter/colorViewModel';
	import ColorCode from '$lib/label/framework/ui/ColorCode.svelte';
	import EditLabelDialog, {
		type EditLabelDialogFormParams,
	} from '$lib/label/framework/ui/EditLabelDialog.svelte';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';
	import { getContext } from 'svelte';
	import { toast } from 'svelte-sonner';
	import Table from '$lib/common/framework/components/ui/table/table.svelte';
	import TableBody from '$lib/common/framework/components/ui/table/table-body.svelte';
	import TableRow from '$lib/common/framework/components/ui/table/table-row.svelte';
	import TableCell from '$lib/common/framework/components/ui/table/table-cell.svelte';

	const labelLoadedStore = getContext<LabelLoadedStore>(LabelLoadedStore.name);
	const labelLoadedState = $derived($labelLoadedStore);
	const { trigger: loadLabel } = labelLoadedStore;
	const label = $derived(labelLoadedState.data);

	const labelUpdatedStore = getContext<LabelUpdatedStore>(LabelUpdatedStore.name);
	const labelUpdatedState = $derived($labelUpdatedStore);
	const { trigger: updateLabel } = labelUpdatedStore;

	const relatedPostsStore = getContext<PostsListedStore>(PostsListedStore.name);
	const relatedPostsState = $derived($relatedPostsStore);

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
			toast.error('Label not found');
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
			toast.success('Label updated successfully');
		} else if (state.isError()) {
			toast.error('Failed to update label', {
				description: state.error.message,
			});
			return false;
		}

		return true;
	}
</script>

<div class="dashboard-container mb-10">
	<div class="flex flex-row items-center justify-between">
		<h1 class="py-16 text-5xl font-bold text-gray-800">Label Details</h1>
		{#key formDefaultValues}
			<EditLabelDialog
				title="Update Label"
				triggerButtonText="Edit"
				disabled={!labelLoadedState.isSuccess() || labelUpdatedState.isLoading()}
				defaultValues={formDefaultValues ?? undefined}
				{onSubmit}
			/>
		{/key}
	</div>
	<Table>
		<TableBody>
			<TableRow>
				<TableCell class="font-medium">ID</TableCell>
				<TableCell>{label?.id}</TableCell>
			</TableRow>
			<TableRow>
				<TableCell class="font-medium">Name</TableCell>
				<TableCell>{label?.name}</TableCell>
			</TableRow>
			<TableRow>
				<TableCell class="font-medium">Color</TableCell>
				<TableCell>
					{#if label}
						<ColorCode color={label.color} />
					{/if}
				</TableCell>
			</TableRow>
			<TableRow>
				<TableCell class="font-medium">Preview</TableCell>
				<TableCell>
					{#if label}
						<PostLabel {label} />
					{/if}
				</TableCell>
			</TableRow>
			<TableRow>
				<TableCell class="font-medium">Related Posts</TableCell>
				<TableCell>
					{#each relatedPostsState.data ?? [] as postInfo (postInfo.id)}
						<p>
							<a href="/dashboard/post/{postInfo.id}" class="underline">
								{postInfo.title}
							</a>
						</p>
					{/each}
				</TableCell>
			</TableRow>
		</TableBody>
	</Table>
</div>
