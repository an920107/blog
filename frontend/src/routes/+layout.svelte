<script lang="ts">
	import Footer from '$lib/common/framework/ui/Footer.svelte';
	import Navbar from '$lib/common/framework/ui/Navbar.svelte';
	import GoogleAnalytics from '$lib/common/framework/ui/GoogleAnalytics.svelte';
	import '../app.css';
	import '@fortawesome/fontawesome-free/css/all.min.css';
	import { Toaster } from '$lib/common/framework/components/ui/sonner';
	import { Container } from '$lib/container';
	import { setContext } from 'svelte';

	const container = new Container(fetch);
	setContext(Container.name, container);
</script>

<GoogleAnalytics />
<svelte:head>
	<meta name="app-version" content={App.__VERSION__} />
</svelte:head>
<div class="min-h-screen">
	<Toaster theme="light" />
	<Navbar />
	<main>
		<slot />
	</main>
</div>
<Footer />
