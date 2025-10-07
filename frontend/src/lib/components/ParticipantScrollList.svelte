<script>
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import SmartSelect from './SmartSelect.svelte';
	import { Button } from '$lib/components/ui/button/index.js';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';

	let { participants, roleItems } = $props();
</script>

<ScrollArea class="max-h-80 w-full rounded-md border">
	<div class="px-4 py-2">
		{#if participants.length === 0}
			<div class="py-2 text-center text-sm text-muted-foreground">No results found</div>
		{:else}
			{#each participants as participant, i}
				<div class="flex items-center justify-between text-sm">
					<div class="flex items-center space-x-2">
						<Avatar.Root>
							<Avatar.Image src={participant.id} alt={participant.name} />
							<Avatar.Fallback>
								{participant.name.slice(0, 2).toUpperCase()}
							</Avatar.Fallback>
						</Avatar.Root>
						<div>{participant.name}</div>
					</div>
					<div class="flex items-center gap-2">
						<SmartSelect type="multiple" label="Role" items={roleItems} />
						<Button variant="destructive">
							<Trash2 />
							Remove
						</Button>
					</div>
				</div>
				{#if i < participants.length - 1}
					<Separator class="my-2" />
				{/if}
			{/each}
		{/if}
	</div>
</ScrollArea>
