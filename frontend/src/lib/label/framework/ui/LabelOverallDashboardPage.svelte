<script lang="ts">
	import TableBody from '$lib/common/framework/components/ui/table/table-body.svelte';
	import TableHead from '$lib/common/framework/components/ui/table/table-head.svelte';
	import TableHeader from '$lib/common/framework/components/ui/table/table-header.svelte';
	import TableRow from '$lib/common/framework/components/ui/table/table-row.svelte';
	import Table from '$lib/common/framework/components/ui/table/table.svelte';
	import { LabelCreatedStore } from '$lib/label/adapter/presenter/labelCreatedStore';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/labelsListedStore';
	import EditLabelDialog, {
		type EditLabelDialogFormParams,
	} from '$lib/label/framework/ui/EditLabelDialog.svelte';
	import { ColorViewModel } from '$lib/label/adapter/presenter/colorViewModel';
	import { getContext, onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import LabelOverallDashboardTabelRow from '$lib/label/framework/ui/LabelOverallDashboardTabelRow.svelte';

	const labelCreatedStore = getContext<LabelCreatedStore>(LabelCreatedStore.name);
	const labelCreatedState = $derived($labelCreatedStore);
	const { trigger: createLabel } = labelCreatedStore;

	const labelsListedStore = getContext<LabelsListedStore>(LabelsListedStore.name);
	const labelsListedState = $derived($labelsListedStore);
	const { trigger: loadLabels } = labelsListedStore;

	async function onSubmit(params: EditLabelDialogFormParams): Promise<boolean> {
		const colorViewModel = ColorViewModel.fromHex(params.color);
		const color = colorViewModel.toEntity();

		const state = await createLabel({
			name: params.name,
			color: color,
		});

		if (state.isSuccess()) {
			loadLabels();
			toast.success(`Label created successfully with ID: ${state.data.id}`);
		} else if (state.isError()) {
			toast.error('Failed to create label', {
				description: state.error.message,
			});
			return false;
		}

		return true;
	}

	onMount(() => loadLabels());
</script>

<div class="dashboard-container mb-10">
	<div class="flex flex-row items-center justify-between">
		<h1 class="py-16 text-5xl font-bold text-gray-800">Label</h1>
		<EditLabelDialog
			title="Create Label"
			triggerButtonText="Create"
			disabled={labelCreatedState.isLoading()}
			{onSubmit}
		/>
	</div>
	<Table>
		<TableHeader>
			<TableRow>
				<TableHead>ID</TableHead>
				<TableHead>Name</TableHead>
				<TableHead>Color</TableHead>
				<TableHead>Preview</TableHead>
			</TableRow>
		</TableHeader>
		<TableBody>
			{#if labelsListedState.isSuccess()}
				{#each labelsListedState.data as label (label.id)}
					<LabelOverallDashboardTabelRow {label} />
				{/each}
			{/if}
		</TableBody>
	</Table>
</div>
