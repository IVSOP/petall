<script lang="ts">
	const { children } = $props();
</script>

<p
	class="mb-3 rounded-md border border-destructive/20 bg-destructive/10 p-3 text-sm text-destructive"
	role="alert"
>
	{@render children()}
</p>
