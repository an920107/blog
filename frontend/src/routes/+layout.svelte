<script>
	import Footer from '$lib/common/framework/ui/Footer.svelte';
	import Navbar from '$lib/common/framework/ui/Navbar.svelte';
	import '../app.css';
	import '@fortawesome/fontawesome-free/css/all.min.css';
</script>

<svelte:head>
	<meta name="app-version" content={App.__VERSION__} />
</svelte:head>
<div class="min-h-screen">
	<Navbar />
	<slot />
</div>
<Footer />
