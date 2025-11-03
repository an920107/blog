<script lang="ts">
	import ErrorPage from '$lib/common/framework/ui/ErrorPage.svelte';
	import generateTitle from '$lib/common/framework/ui/generateTitle';
</script>

<svelte:head>
	<title>{generateTitle('404 Not Found')}</title>
</svelte:head>

<ErrorPage />
