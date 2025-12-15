<script lang="ts">
	import { DrawerConfiguredStore } from '$lib/common/adapter/presenter/drawerConfiguredStore';
	import { getContext } from 'svelte';
	import { fade, scale } from 'svelte/transition';
	import { cn } from '$lib/common/framework/components/utils';

	const drawerConfiguredStore = getContext<DrawerConfiguredStore>(DrawerConfiguredStore.name);
	const drawerConfiguredState = $derived($drawerConfiguredStore);
	const drawerViewModel = $derived(drawerConfiguredState?.data);
	const { trigger: configureDrawer } = drawerConfiguredStore;

	function setDrawerOpen(isOpen: boolean) {
		if (drawerViewModel) {
			configureDrawer(drawerViewModel.copyWith({ isOpen }));
		}
	}

	$effect(() => {
		if (drawerViewModel?.isOpen) {
			// Save current scroll position and prevent scrolling
			const scrollY = window.scrollY;
			document.body.style.position = 'fixed';
			document.body.style.top = `-${scrollY}px`;
			document.body.style.width = '100%';
			document.body.style.overflow = 'hidden';
		} else {
			// Restore scroll position and allow scrolling
			const scrollY = document.body.style.top;
			document.body.style.position = '';
			document.body.style.top = '';
			document.body.style.width = '';
			document.body.style.overflow = '';
			if (scrollY) {
				window.scrollTo(0, parseInt(scrollY || '0') * -1);
			}
		}
	});

	function drawerTransition(node: Element) {
		const options = scale(node, { start: 0.8 });
		const originalCss = options.css;
		options.css = (t: number, u: number) => {
			if (!originalCss) {
				return '';
			}
			return originalCss(t, u).replace('scale(', 'scaleX(');
		};
		return options;
	}
</script>

{#if drawerViewModel?.isOpen}
	<div transition:fade class="fixed inset-0 z-[99] bg-black/10 backdrop-blur-md">
		<button
			title="Close drawer"
			class="h-full w-full cursor-default!"
			onclick={() => setDrawerOpen(false)}
		></button>
	</div>
	<div
		transition:drawerTransition
		class={cn(
			'fixed top-0 right-0 bottom-0 z-[100] origin-right rounded-l-3xl bg-white',
			'flex w-2xs max-w-5/6 flex-col gap-y-8 px-4 py-5'
		)}
	>
		<div class="flex flex-row items-center justify-between">
			<img class="size-10" src="/icon/logo-light.svg" alt="SquidSpirit" />
			<button type="button" title="Close drawer" onclick={() => setDrawerOpen(false)}>
				<i class="fa-solid fa-xmark size-2"></i>
			</button>
		</div>
		{#if drawerViewModel?.actions}
			<div transition:fade class="flex flex-col gap-y-2">
				{@render drawerViewModel.actions()}
			</div>
		{/if}
		{#if drawerViewModel?.content}
			<hr transition:fade />
			<div transition:fade>
				{@render drawerViewModel.content()}
			</div>
		{/if}
	</div>
{/if}
