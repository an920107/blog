<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/common/framework/components/ui/button/index';
	import type { DashboardLink } from '$lib/dashboard/framework/ui/dashboardLink';

	const { link }: { link: DashboardLink } = $props();

	const isSelected = $derived.by(() => {
		const pathname = page.url.pathname;
		return pathname === link.href || pathname.startsWith(link.href + '/');
	});
</script>

<a href={link.href}>
	<Button variant={isSelected ? 'outline' : 'ghost'} class="w-full">
		<span class="w-full text-left">
			{link.label}
		</span>
	</Button>
</a>
