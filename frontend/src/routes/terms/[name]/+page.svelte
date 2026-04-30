<script lang="ts">
	import TermsPage from '$lib/terms/framework/ui/TermsPage.svelte';

	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
</script>

<TermsPage title={data.title} content={data.content} />
