<script lang="ts">
	import type { DashboardLink } from '$lib/dashboard/framework/ui/dashboardLink';
	import DashboardLinkButton from '$lib/dashboard/framework/ui/DashboardLinkButton.svelte';
	import { Strings } from '$lib/strings';

	const { links }: { links: DashboardLink[] } = $props();
</script>

<div class="w-3xs border-e border-true-gray-300 p-8">
	<div class="mb-3 font-bold">{Strings.DASHBOARD}</div>
	<div class="flex flex-col">
		{#each links as link (link.href)}
			<DashboardLinkButton {link} />
		{/each}
	</div>
</div>
