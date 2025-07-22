<script lang="ts">
	let { label, link, isSelected } = $props();
</script>

<div class="rounded px-1.5 {isSelected ? 'bg-blue-600' : 'bg-transparent'}">
	<a class="font-extrabold {isSelected ? 'text-white' : 'text-gray-800'}" href={link}>
		{label}
	</a>
</div>
