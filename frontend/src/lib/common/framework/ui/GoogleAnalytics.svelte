<script>
	import { Environment } from '$lib/environment';
	import { onMount } from 'svelte';

	onMount(() => {
		const gaMeasurementId = Environment.GA_MEASUREMENT_ID;

		if (!gaMeasurementId.startsWith('G-')) {
			return;
		}

		if (document.querySelector(`script[src*="${gaMeasurementId}"]`)) {
			return;
		}

		const gaScript = document.createElement('script');
		gaScript.async = true;
		gaScript.src = `https://www.googletagmanager.com/gtag/js?id=${gaMeasurementId}`;
		document.head.appendChild(gaScript);

		window.dataLayer = window.dataLayer || [];
		function gtag() {
			window.dataLayer.push(arguments);
		}
		gtag('js', new Date());
		gtag('config', gaMeasurementId);
	});
</script>
