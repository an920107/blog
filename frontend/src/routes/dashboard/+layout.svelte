<script lang="ts">
	import type { DashboardLink } from '$lib/dashboard/framework/ui/dashboardLink';
	import DashboardNavbar from '$lib/dashboard/framework/ui/DashboardNavbar.svelte';
	import { Strings } from '$lib/strings';

	import type { LayoutProps } from './$types';

	const { children }: LayoutProps = $props();

	const links: DashboardLink[] = [
		{ label: Strings.DASHBOARD_POST, href: '/dashboard/post' },
		{ label: Strings.DASHBOARD_LABEL, href: '/dashboard/label' },
		{ label: Strings.DASHBOARD_IMAGE, href: '/dashboard/image' },
	];
</script>

<div class="grid min-h-content-height grid-cols-[auto_1fr]">
	<DashboardNavbar {links} />
	{@render children()}
</div>
