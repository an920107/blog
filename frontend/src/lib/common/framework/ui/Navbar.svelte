<script module lang="ts">
	export interface NavigationActionProps {
		href: string;
		label: string;
	}
</script>

<script lang="ts">
	import { page } from '$app/state';
	import { cn } from '$lib/common/framework/components/utils';
	import { getContext, onDestroy, onMount } from 'svelte';
	import { DrawerConfiguredStore } from '$lib/common/adapter/presenter/drawerConfiguredStore';

	const { actions }: { actions: NavigationActionProps[] } = $props();

	const drawerConfiguredStore = getContext<DrawerConfiguredStore>(DrawerConfiguredStore.name);
	const drawerConfiguredState = $derived($drawerConfiguredStore);
	const drawerViewModel = $derived(drawerConfiguredState?.data);
	const { trigger: configureDrawer } = drawerConfiguredStore;

	function setDrawerOpen(isOpen: boolean) {
		if (drawerViewModel) {
			configureDrawer(drawerViewModel.copyWith({ isOpen }));
		}
	}

	let isVisible = $state(true);
	let lastScrollY = $state(0);

	onMount(() => {
		function handleScroll() {
			const currentScrollY = window.scrollY;

			// If at top, always show
			if (currentScrollY <= 0) {
				isVisible = true;
				lastScrollY = currentScrollY;
				return;
			}

			// Show when scrolling up, hide when scrolling down
			if (currentScrollY < lastScrollY) {
				isVisible = true;
			} else if (currentScrollY > lastScrollY && currentScrollY > 100) {
				// Only hide after scrolling down 100px
				isVisible = false;
			}

			lastScrollY = currentScrollY;
		}

		window.addEventListener('scroll', handleScroll, { passive: true });
		onDestroy(() => {
			window.removeEventListener('scroll', handleScroll);
		});
	});

	onMount(() => {
		if (drawerViewModel) {
			configureDrawer(drawerViewModel.copyWith({ actions: actionLinks }));
		}
	});
</script>

<nav
	class={cn(
		'fixed top-0 right-0 left-0 z-50 border-b border-gray-300 bg-white',
		'transition-transform duration-300',
		isVisible ? 'translate-y-0' : '-translate-y-full'
	)}
>
	<div
		class={cn(
			'mx-auto h-toolbar-height max-w-screen-xl px-4 md:px-6',
			'flex flex-row items-center justify-between'
		)}
	>
		<a class="flex flex-row items-center gap-x-2" href="/">
			<img class="mt-1 size-10" src="/icon/logo-light.svg" alt="SquidSpirit" />
			<span class="text-2xl font-black text-gray-800">魚之魷魂</span>
		</a>

		<button type="button" title="Open drawer" class="md:hidden" onclick={() => setDrawerOpen(true)}>
			<i class="fa-solid fa-bars size-2"></i>
		</button>
		<div class="flex flex-row items-center gap-x-6 max-md:hidden">
			{@render actionLinks()}
		</div>
	</div>
</nav>

{#snippet actionLinks()}
	{#each actions as action (action.href)}
		{@const { href, label } = action}
		{@const isSelected = page.url.pathname === href || page.url.pathname.startsWith(href + '/')}
		<a
			{href}
			class={cn(
				'rounded-[0.375rem] px-2.5 py-1 font-extrabold md:rounded md:px-1.5 md:py-0',
				isSelected ? 'bg-blue-600 text-white' : 'bg-transparent text-gray-800',
				'transition-colors duration-200',
				isSelected ? 'hover:bg-blue-700' : 'hover:bg-gray-100'
			)}
			onclick={() => {
				// Reseting scroll position by set `top` as 0 since drawer prevents scrolling
				document.body.style.top = '0px';
				setTimeout(() => setDrawerOpen(false), 200);
			}}
		>
			{label}
		</a>
	{/each}
{/snippet}
