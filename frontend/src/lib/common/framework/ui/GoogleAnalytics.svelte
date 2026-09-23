<script>
	import { onMount } from 'svelte';

	import { Environment } from '$lib/environment';

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
		/** @param {...unknown} args */
		function gtag(...args) {
			window.dataLayer.push(args);
		}
		gtag('js', new Date());
		gtag('config', gaMeasurementId);
	});
</script>
