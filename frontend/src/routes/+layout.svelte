<script lang="ts">
	import '@fortawesome/fontawesome-free/css/all.min.css';
	import '../app.css';

	import { setContext } from 'svelte';

	import { resolve } from '$app/paths';
	import { DrawerConfiguredStore } from '$lib/common/adapter/presenter/drawerConfiguredStore';
	import { Toaster } from '$lib/common/framework/components/ui/sonner';
	import Footer from '$lib/common/framework/ui/Footer.svelte';
	import GoogleAdsense from '$lib/common/framework/ui/GoogleAdsense.svelte';
	import GoogleAnalytics from '$lib/common/framework/ui/GoogleAnalytics.svelte';
	import Navbar, { type NavigationActionProps } from '$lib/common/framework/ui/Navbar.svelte';
	import NavigationDrawer from '$lib/common/framework/ui/NavigationDrawer.svelte';
	import { Container } from '$lib/container';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/labelsListedStore';
	import PostSearchLauncher from '$lib/post/framework/ui/PostSearchLauncher.svelte';

	const container = new Container(fetch);
	setContext(Container.name, container);

	const drawerConfiguredStore = container.createDrawerConfiguredStore();
	setContext(DrawerConfiguredStore.name, drawerConfiguredStore);

	// Provided globally so the navbar search dialog works on every page.
	const labelsListedStore = container.createLabelsListedStore();
	setContext(LabelsListedStore.name, labelsListedStore);

	const navigationActions: NavigationActionProps[] = [
		{ href: resolve('/'), label: '首頁' },
		{ href: resolve('/post'), label: '文章' },
	];
</script>

<GoogleAnalytics />
<GoogleAdsense />
<svelte:head>
	<meta name="app-version" content={App.__VERSION__} />
</svelte:head>
<div class="min-h-screen">
	<Toaster theme="light" />
	<Navbar actions={navigationActions}>
		{#snippet search()}
			<PostSearchLauncher />
		{/snippet}
	</Navbar>
	<NavigationDrawer />
	<main class="pt-toolbar-height">
		<slot />
	</main>
</div>
<Footer />
