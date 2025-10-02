<script>
	import * as Command from '$lib/components/ui/command/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	let { open = $bindable(), participants } = $props();
</script>

<Command.Dialog bind:open>
	<Command.Input placeholder="Find participant..." />
	<Command.List>
		<ScrollArea class="h-70">
			<Command.Empty class="text-center text-sm text-muted-foreground">
				No results found
			</Command.Empty>
			<Command.Group>
				{#each participants as participant}
					<Command.Item class=" flex items-center text-sm">
						<div class="flex items-center space-x-2">
							<Avatar.Root>
								<Avatar.Image src={participant.id} alt={participant.name} />
								<Avatar.Fallback>CN</Avatar.Fallback>
							</Avatar.Root>
							<div>{participant.name}</div>
						</div>
					</Command.Item>
				{/each}
			</Command.Group>
		</ScrollArea>
	</Command.List>
</Command.Dialog>
