<script lang="ts">
	import { getContext, onMount } from 'svelte';
	import { toast } from 'svelte-sonner';

	import { resolve } from '$app/paths';
	import { LabelCreatedStore } from '$lib/label/adapter/presenter/store/labelCreatedStore';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/store/labelsListedStore';
	import { ColorViewModel } from '$lib/label/adapter/presenter/viewModel/colorViewModel';
	import ColorCode from '$lib/label/framework/ui/ColorCode.svelte';
	import EditLabelDialog, {
		type EditLabelDialogFormParams,
	} from '$lib/label/framework/ui/EditLabelDialog.svelte';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';
	import { TableCell } from '$lib/shared/framework/components/ui/table';
	import Table from '$lib/shared/framework/components/ui/table/table.svelte';
	import TableBody from '$lib/shared/framework/components/ui/table/table-body.svelte';
	import TableHead from '$lib/shared/framework/components/ui/table/table-head.svelte';
	import TableHeader from '$lib/shared/framework/components/ui/table/table-header.svelte';
	import TableRow from '$lib/shared/framework/components/ui/table/table-row.svelte';
	import { Strings } from '$lib/strings';

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
			toast.success(`${Strings.LABEL_CREATED_PREFIX} ${state.data.id}`);
		} else if (state.isError()) {
			toast.error(Strings.FAILED_TO_CREATE_LABEL, {
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
		<h1 class="py-16 text-5xl font-bold text-gray-800">{Strings.DASHBOARD_LABEL}</h1>
		<EditLabelDialog
			title={Strings.CREATE_LABEL}
			triggerButtonText={Strings.CREATE}
			disabled={labelCreatedState.isLoading()}
			{onSubmit}
		/>
	</div>
	<Table>
		<TableHeader>
			<TableRow>
				<TableHead>{Strings.ID}</TableHead>
				<TableHead>{Strings.NAME}</TableHead>
				<TableHead>{Strings.COLOR}</TableHead>
				<TableHead>{Strings.PREVIEW}</TableHead>
			</TableRow>
		</TableHeader>
		<TableBody>
			{#each labelsListedState.data ?? [] as label (label.id)}
				<TableRow>
					<TableHead>
						<a
							href={resolve('/dashboard/label/[id]', { id: label.id.toString() })}
							class="underline">{label.id}</a
						>
					</TableHead>
					<TableCell>
						<span class="text-wrap"> {label.name}</span>
					</TableCell>
					<TableCell>
						<ColorCode color={label.color} />
					</TableCell>
					<TableCell>
						<PostLabel {label} />
					</TableCell>
				</TableRow>
			{/each}
		</TableBody>
	</Table>
</div>
