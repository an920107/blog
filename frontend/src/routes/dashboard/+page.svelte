<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { resolve } from '$app/paths';

	onMount(() => {
		if (page.url.pathname === '/dashboard') {
			goto(resolve('/dashboard/post'));
		}
	});
</script>
