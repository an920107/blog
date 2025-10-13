<script lang="ts">
	import type { LabelViewModel } from '$lib/post/adapter/presenter/labelViewModel';

	const { label }: { label: LabelViewModel } = $props();
</script>

<div
	class="flex flex-row items-center gap-x-1 rounded-full px-2 py-0.5"
	style="background-color: {label.color.hex};"
>
	<div class="size-2 rounded-full" style="background-color: {label.color.darken(0.2).hex};"></div>
	<span class="text-xs">{label.name}</span>
</div>
