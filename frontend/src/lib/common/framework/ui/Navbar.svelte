<script lang="ts">
	import { page } from '$app/state';
	import NavbarAction from '$lib/common/framework/ui/NavbarAction.svelte';
</script>

<div class="border-b border-gray-300">
	<div
		class="mx-auto flex h-toolbar-height max-w-screen-xl flex-row items-center justify-between px-4 md:px-6"
	>
		<a class="flex flex-row items-center gap-x-2" href="/">
			<img class="mt-1 size-10" src="/icon/logo-light.svg" alt="SquidSpirit" />
			<span class="text-2xl font-black text-gray-800">魚之魷魂</span>
		</a>
		<div class="flex flex-row items-center gap-x-6">
			<NavbarAction label="首頁" link="/" isSelected={page.url.pathname === '/'} />
			<NavbarAction
				label="文章"
				link="/post"
				isSelected={page.url.pathname === '/post' || page.url.pathname.startsWith('/post/')}
			/>
		</div>
	</div>
</div>
