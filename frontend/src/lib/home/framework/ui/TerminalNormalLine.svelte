<script lang="ts">
	const { text }: { text: string } = $props();
</script>

<div class="flex w-full flex-row gap-x-1.5 md:gap-x-2">
	<span class="text-green-400">❯</span>
	<span>{text}</span>
</div>
