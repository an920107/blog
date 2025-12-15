<script lang="ts">
	import { cn } from '$lib/common/framework/components/utils';

	let { onSearch }: { onSearch: (keyword: string) => void } = $props();

	let keyword = $state('');
	let debounceTimer: ReturnType<typeof setTimeout> | undefined;

	function handleInput(event: Event) {
		const target = event.target as HTMLInputElement;
		keyword = target.value;

		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}

		debounceTimer = setTimeout(() => {
			onSearch(keyword);
		}, 1000);
	}

	function handleChange() {
		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}
		onSearch(keyword);
	}

	function handleClear() {
		keyword = '';
		if (debounceTimer) {
			clearTimeout(debounceTimer);
		}
		onSearch(keyword);
	}
</script>

<div class="w-full pt-2 pb-9 md:mx-auto md:max-w-[40rem] md:pt-4 md:pb-20">
	<div
		class={cn(
			'flex w-full flex-row items-center gap-x-2 px-4 py-2',
			'rounded-full border border-gray-300',
			'transition-shadow duration-300',
			'has-[:hover]:shadow-[0_0_20px_rgba(59,130,246,0.5)]',
			'has-[:focus]:shadow-[0_0_20px_rgba(59,130,246,0.5)]'
		)}
	>
		<i class="fa-solid fa-magnifying-glass text-gray-400"></i>
		<input
			class="w-full focus:outline-none"
			type="text"
			placeholder="搜尋文章標題或內容......"
			value={keyword}
			oninput={handleInput}
			onchange={handleChange}
		/>
		{#if keyword}
			<button
				onclick={handleClear}
				title="清除搜尋內容"
				class="text-gray-400 hover:text-gray-600"
				type="button"
			>
				<i class="fa-solid fa-xmark"></i>
			</button>
		{/if}
	</div>
</div>
