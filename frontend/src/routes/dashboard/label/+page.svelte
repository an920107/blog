<script lang="ts">
	import { Container } from '$lib/container';
	import { LabelCreatedStore } from '$lib/label/adapter/presenter/labelCreatedStore';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/labelsListedStore';
	import LabelOverallDashboardPage from '$lib/label/framework/ui/LabelOverallDashboardPage.svelte';
	import { getContext, setContext } from 'svelte';
	import type { PageProps } from './$types';
	import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';

	const { data }: PageProps = $props();
	const container = getContext<Container>(Container.name);

	const labelCreatedStore = container.createLabelCreatedStore();
	setContext(LabelCreatedStore.name, labelCreatedStore);

	const initialData = data.dehydratedData?.map((label) => LabelViewModel.rehydrate(label));
	const labelsListedStore = container.createLabelsListedStore(initialData);
	setContext(LabelsListedStore.name, labelsListedStore);
</script>

<LabelOverallDashboardPage />
