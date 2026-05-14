<script lang="ts">
	import { onMount } from 'svelte';

	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';

	onMount(() => {
		if (page.url.pathname === '/dashboard') {
			goto(resolve('/dashboard/post'));
		}
	});
</script>
