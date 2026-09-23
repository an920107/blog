<script>
	import { onMount } from 'svelte';

	import { Environment } from '$lib/environment';

	const adsenseClientId = Environment.ADSENSE_CLIENT_ID;
	const isAdsenseEnabled = adsenseClientId.startsWith('ca-pub-');

	onMount(() => {
		if (!isAdsenseEnabled) {
			return;
		}

		if (document.querySelector('script[src*="adsbygoogle"]')) {
			return;
		}

		const adsenseScript = document.createElement('script');
		adsenseScript.async = true;
		adsenseScript.crossOrigin = 'anonymous';
		adsenseScript.src = `https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=${adsenseClientId}`;
		document.head.appendChild(adsenseScript);
	});
</script>

<svelte:head>
	{#if isAdsenseEnabled}
		<meta name="google-adsense-account" content={adsenseClientId} />
	{/if}
</svelte:head>
