<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { onMount } from 'svelte';

	onMount(() => {
		if (page.url.pathname === '/dashboard') {
			goto('/dashboard/post');
		}
	});
</script>
