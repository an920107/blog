<script lang="ts">
	import { TableCell, TableRow } from '$lib/common/framework/components/ui/table';
	import type { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
	import ColorCode from '$lib/label/framework/ui/ColorCode.svelte';
	import PostLabel from '$lib/label/framework/ui/PostLabel.svelte';

	const { label }: { label: LabelViewModel } = $props();
</script>

<TableRow>
	<TableCell>
		<a href={`/dashboard/label/${label.id}`} class="font-medium underline">
			{label.id}
		</a>
	</TableCell>
	<TableCell><span class="text-wrap">{label.name}</span></TableCell>
	<TableCell>
		<ColorCode color={label.color} />
	</TableCell>
	<TableCell>
		<PostLabel {label} />
	</TableCell>
</TableRow>
