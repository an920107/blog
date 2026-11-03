<script lang="ts">
	const { message }: { message?: string | null } = $props();
</script>

<p class="text-sm text-destructive" hidden={!message}>{message}</p>
