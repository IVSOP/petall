<script lang="ts">
	import * as Command from '$lib/components/ui/command/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import UserPlus from '@tabler/icons-svelte/icons/user-plus';

	let { open = $bindable(), members, type, title } = $props();

	function addMember(member_id: String) {
		if (type === 'user') {
			console.log('Add user ' + member_id);
		} else if (type === 'manager') {
			console.log('Add manager ' + member_id);
		} else {
			throw new Error('Invalid type');
		}
	}
</script>

<Command.Dialog bind:open>
	<Command.Input placeholder={title} />
	<Command.List>
		<ScrollArea class="h-70">
			<Command.Empty class="text-center text-sm text-muted-foreground">
				No results found
			</Command.Empty>
			<Command.Group>
				{#each members as member}
					<Command.Item class="flex items-center justify-between text-sm">
						<div class="flex items-center space-x-2">
							<Avatar.Root>
								<Avatar.Fallback class="bg-primary/10 text-sm font-medium">
									{member.name.slice(0,2).toUpperCase()}
								</Avatar.Fallback>
							</Avatar.Root>
							<div>{member.name}</div>
						</div>

						<Button
							class="cursor-pointer"
							onclick={() => addMember(member.id)}
						>
							<UserPlus class="text-white" />
							Add {type}
						</Button>
					</Command.Item>
				{/each}
			</Command.Group>
		</ScrollArea>
	</Command.List>
</Command.Dialog>

