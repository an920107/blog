<script lang="ts">
	import { getContext, onMount, setContext } from 'svelte';
	import type { LayoutProps } from './$types';
	import ErrorPage from '$lib/common/framework/ui/ErrorPage.svelte';
	import DashboardNavbar from '$lib/dashboard/framework/ui/DashboardNavbar.svelte';
	import type { DashboardLink } from '$lib/dashboard/framework/ui/dashboardLink';
	import { Container } from '$lib/container';
	import { AuthLoadedStore } from '$lib/auth/adapter/presenter/authLoadedStore';

	const { children }: LayoutProps = $props();
	const container = getContext<Container>(Container.name);
	const store = container.createAuthLoadedStore();
	setContext(AuthLoadedStore.name, store);

	const state = $derived($store);
	const { trigger: loadAuthentication } = store;

	const isAuthenticated = $derived(state.isSuccess() && state.data.isAuthenticated);

	const links: DashboardLink[] = [
		{ label: 'Post', href: '/dashboard/post' },
		{ label: 'Label', href: '/dashboard/label' },
		{ label: 'Image', href: '/dashboard/image' },
	];

	onMount(() => loadAuthentication());
</script>

{#if state.isIdle() || state.isLoading()}
	<div></div>
{:else if !isAuthenticated}
	<ErrorPage />
{:else}
	<div class="grid min-h-content-height grid-cols-[auto_1fr]">
		<DashboardNavbar {links} />
		{@render children()}
	</div>
{/if}
