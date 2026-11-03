<script lang="ts">
	import { setContext } from 'svelte';

	import { LabelCreatedStore } from '$lib/label/adapter/presenter/store/labelCreatedStore';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/store/labelsListedStore';
	import { LabelViewModel } from '$lib/label/adapter/presenter/viewModel/labelViewModel';
	import LabelOverallDashboardPage from '$lib/label/framework/ui/LabelOverallDashboardPage.svelte';
	import { getContainer } from '$lib/shared/framework/ui/containerContext';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const container = getContainer();

	const labelCreatedStore = container.createLabelCreatedStore();
	setContext(LabelCreatedStore.name, labelCreatedStore);

	const getInitialData = () => data.dehydratedData?.map((label) => LabelViewModel.rehydrate(label));
	const labelsListedStore = container.createLabelsListedStore(getInitialData());
	setContext(LabelsListedStore.name, labelsListedStore);
</script>

<LabelOverallDashboardPage />
