<script lang="ts">
	import type { ColorViewModel } from '$lib/label/adapter/presenter/colorViewModel';

	const { color }: { color: ColorViewModel } = $props();
</script>

<div class="flex items-center gap-2">
	<div class="size-4 rounded-full" style="background-color: {color.hex};"></div>
	<span class="font-mono text-sm">{color.hex}</span>
</div>
