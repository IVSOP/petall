<script lang="ts">
	import { Input } from '$lib/components/ui/input/index.js';

	export let value: string;
</script>

<div class="flex w-full items-center pb-4">
	<Input
		bind:value
		placeholder="Filter communities"
		class="w-full text-base sm:text-lg md:text-lg lg:text-xl"
	/>
</div>
