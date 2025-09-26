<script lang="ts">
	import { Input } from '$lib/components/ui/input/index.js';
	import Search from '@lucide/svelte/icons/search';

	let { value = $bindable(), placeholder, ...props } = $props();
</script>

<div class="relative">
	<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
		<Search class="h-4 w-4 text-muted-foreground" />
	</div>
	<Input bind:value {...props} {placeholder} class="flex-1 pl-9 text-sm md:text-base" />
</div>
