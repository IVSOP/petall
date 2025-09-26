<script lang="ts">
	import * as Select from '$lib/components/ui/select/index.js';

	let { label, items, type } = $props();
</script>

<Select.Root {type}>
	<Select.Trigger class="text-sm md:text-base">
		{label}
	</Select.Trigger>
	<Select.Content>
		<!--
        <Select.Label class="text-sm">{label}</Select.Label>
		<Select.Separator />
        -->
		{#each items as item}
			<Select.Item value={item.value}>{item.label}</Select.Item>
		{/each}
	</Select.Content>
</Select.Root>
