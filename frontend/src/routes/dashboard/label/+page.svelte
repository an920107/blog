<script lang="ts">
	import { getContext, setContext } from 'svelte';

	import { Container } from '$lib/container';
	import { LabelCreatedStore } from '$lib/label/adapter/presenter/labelCreatedStore';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/labelsListedStore';
	import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
	import LabelOverallDashboardPage from '$lib/label/framework/ui/LabelOverallDashboardPage.svelte';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const container = getContext<Container>(Container.name);

	const labelCreatedStore = container.createLabelCreatedStore();
	setContext(LabelCreatedStore.name, labelCreatedStore);

	const getInitialData = () => data.dehydratedData?.map((label) => LabelViewModel.rehydrate(label));
	const labelsListedStore = container.createLabelsListedStore(getInitialData());
	setContext(LabelsListedStore.name, labelsListedStore);
</script>

<LabelOverallDashboardPage />
