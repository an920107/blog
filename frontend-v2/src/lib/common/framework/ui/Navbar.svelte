<script lang="ts">
	import { page } from '$app/state';
	import Action from '$lib/common/framework/ui/Action.svelte';
</script>

<div class="border-b border-gray-300">
	<div
		class="h-toolbar-height mx-auto flex max-w-screen-xl flex-row items-center justify-between px-4 md:px-6"
	>
		<a class="flex flex-row items-center gap-x-2" href="/">
			<img class="mt-1 size-10" src="/icon/logo-light.svg" alt="SquidSpirit" />
			<span class="text-2xl font-black text-gray-800">魚之魷魂</span>
		</a>
		<div class="flex flex-row items-center gap-x-6">
			<Action label="首頁" link="/" isSelected={page.url.pathname === '/'} />
		</div>
	</div>
</div>
