<script>
	import Footer from '$lib/common/framework/ui/Footer.svelte';
	import Navbar from '$lib/common/framework/ui/Navbar.svelte';
	import '../app.css';
	import '@fortawesome/fontawesome-free/css/all.min.css';
</script>

<div class="min-h-screen">
	<Navbar />
	<slot />
</div>
<Footer />
