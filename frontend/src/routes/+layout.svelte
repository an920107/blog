<script lang="ts">
	import Footer from '$lib/common/framework/ui/Footer.svelte';
	import Navbar, { type NavigationActionProps } from '$lib/common/framework/ui/Navbar.svelte';
	import GoogleAnalytics from '$lib/common/framework/ui/GoogleAnalytics.svelte';
	import '../app.css';
	import '@fortawesome/fontawesome-free/css/all.min.css';
	import { Toaster } from '$lib/common/framework/components/ui/sonner';
	import { Container } from '$lib/container';
	import { setContext } from 'svelte';
	import { DrawerConfiguredStore } from '$lib/common/adapter/presenter/drawerConfiguredStore';
	import NavigationDrawer from '$lib/common/framework/ui/NavigationDrawer.svelte';

	const container = new Container(fetch);
	setContext(Container.name, container);

	const drawerConfiguredStore = container.createDrawerConfiguredStore();
	setContext(DrawerConfiguredStore.name, drawerConfiguredStore);

	const navigationActions: NavigationActionProps[] = [
		{ href: '/', label: '首頁' },
		{ href: '/post', label: '文章' },
	];
</script>

<GoogleAnalytics />
<svelte:head>
	<meta name="app-version" content={App.__VERSION__} />
</svelte:head>
<div class="min-h-screen">
	<Toaster theme="light" />
	<Navbar actions={navigationActions} />
	<NavigationDrawer />
	<main class="pt-toolbar-height">
		<slot />
	</main>
</div>
<Footer />
