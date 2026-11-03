<script lang="ts">
	import '@fortawesome/fontawesome-free/css/all.min.css';
	import '../app.css';

	import { setContext } from 'svelte';

	import { resolve } from '$app/paths';
	import { Container } from '$lib/container';
	import { LabelsListedStore } from '$lib/label/adapter/presenter/store/labelsListedStore';
	import PostSearchLauncher from '$lib/post/framework/ui/PostSearchLauncher.svelte';
	import { DrawerConfiguredStore } from '$lib/shared/adapter/presenter/store/drawerConfiguredStore';
	import { Toaster } from '$lib/shared/framework/components/ui/sonner';
	import { setContainer } from '$lib/shared/framework/ui/containerContext';
	import Footer from '$lib/shared/framework/ui/Footer.svelte';
	import GoogleAdsense from '$lib/shared/framework/ui/GoogleAdsense.svelte';
	import GoogleAnalytics from '$lib/shared/framework/ui/GoogleAnalytics.svelte';
	import Navbar, { type NavigationActionProps } from '$lib/shared/framework/ui/Navbar.svelte';
	import NavigationDrawer from '$lib/shared/framework/ui/NavigationDrawer.svelte';
	import { Strings } from '$lib/strings';

	const container = new Container(fetch);
	setContainer(container);

	const drawerConfiguredStore = container.createDrawerConfiguredStore();
	setContext(DrawerConfiguredStore.name, drawerConfiguredStore);

	// Provided globally so the navbar search dialog works on every page.
	const labelsListedStore = container.createLabelsListedStore();
	setContext(LabelsListedStore.name, labelsListedStore);

	const navigationActions: NavigationActionProps[] = [
		{ href: resolve('/'), label: Strings.HOME },
		{ href: resolve('/post'), label: Strings.POST },
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
