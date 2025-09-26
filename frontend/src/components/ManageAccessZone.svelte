<script>
	import SearchBar from './SearchBar.svelte';
	import * as Command from '$lib/components/ui/command/index.js';
	import SquarePlus from '@lucide/svelte/icons/square-plus';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import Trash2 from '@lucide/svelte/icons/trash-2';

	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import SmartSelect from './SmartSelect.svelte';

	const tags = Array.from({ length: 50 }).map((_, i, a) => `v1.2.0-beta.${a.length - i}`);
	let filterParticipantEmail = $state('');

	let roleItems = [
		{ value: 'user', label: 'User' },
		{ value: 'manager', label: 'Manager' }
	];

	let open = $state(false);

	let participants = [
		{ name: 'Digo', image: 'https://github.com/DigoqueDigo.png', roles: ['user', 'manager'] },
		{ name: 'IVSOP', image: 'https://github.com/ivsop.png', roles: ['user', 'manager'] },
		{ name: 'Chico', image: 'https://github.com/chicoferreira.png', roles: ['user', 'manager'] },
		{ name: 'Pedro', image: 'https://github.com/pedroocarvalhoo.png', roles: ['user', 'manager'] },
		{ name: 'Digo1', image: 'https://github.com/DigoqueDigo.png', roles: ['user', 'manager'] },
		{ name: 'IVSOP1', image: 'https://github.com/ivsop.png', roles: ['user', 'manager'] },
		{ name: 'Chico1', image: 'https://github.com/chicoferreira.png', roles: ['user', 'manager'] },
		{ name: 'Pedro1', image: 'https://github.com/pedroocarvalhoo.png', roles: ['user', 'manager'] },
		{ name: 'Digo2', image: 'https://github.com/DigoqueDigo.png', roles: ['user', 'manager'] },
		{ name: 'IVSOP2', image: 'https://github.com/ivsop.png', roles: ['user', 'manager'] },
		{ name: 'Chico2', image: 'https://github.com/chicoferreira.png', roles: ['user', 'manager'] },
		{ name: 'Pedro2', image: 'https://github.com/pedroocarvalhoo.png', roles: ['user', 'manager'] },
		{ name: 'Digo3', image: 'https://github.com/DigoqueDigo.png', roles: ['user', 'manager'] },
		{ name: 'IVSOP3', image: 'https://github.com/ivsop.png', roles: ['user', 'manager'] },
		{ name: 'Chico3', image: 'https://github.com/chicoferreira.png', roles: ['user', 'manager'] },
		{ name: 'Pedro3', image: 'https://github.com/pedroocarvalhoo.png', roles: ['user', 'manager'] }
	];
</script>

<div class="grid gap-4">
	<div class="text-w-full rounded-md border bg-muted px-4 py-2 text-lg font-semibold md:text-xl">
		Manage access
	</div>

	<div class="flex flex-col gap-2 md:flex-row md:items-center">
		<div class="order-2 w-full md:order-none">
			<SearchBar bind:value={filterParticipantEmail} placeholder="Find participant..." />
		</div>
		<div class="order-1 md:order-none">
			<Button
				onclick={() => (open = !open)}
				variant="outline"
				class=" w-full bg-green-600 text-sm text-white hover:bg-green-700 hover:text-white focus:outline-none"
			>
				<SquarePlus class="text-white" />
				Add participant
			</Button>
		</div>
	</div>
	<ScrollArea class="h-80 w-full rounded-md border">
		<div class="px-4 py-2">
			{#each participants as participant, i}
				<div class="flex items-center justify-between text-sm">
					<div class="flex items-center space-x-2">
						<Avatar.Root>
							<Avatar.Image src={participant.image} alt={participant.name} />
							<Avatar.Fallback>CN</Avatar.Fallback>
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
		</div>
	</ScrollArea>

	<Command.Dialog bind:open>
		<Command.Input placeholder="Find participant..." />

		<Command.List>
            <ScrollArea class="h-70">
				<Command.Empty>No results found.</Command.Empty>
				<Command.Group>
					{#each participants as participant}
						<Command.Item class=" flex items-center justify-between text-sm">
							<div class="flex items-center space-x-2">
								<Avatar.Root>
									<Avatar.Image src={participant.image} alt={participant.name} />
									<Avatar.Fallback>CN</Avatar.Fallback>
								</Avatar.Root>
								<div>{participant.name}</div>
							</div>
							<Button
								variant="outline"
								class="bg-green-600 text-white hover:bg-green-700 hover:text-white focus:outline-none"
							>
								<SquarePlus class="text-white" />
								Add
							</Button>
						</Command.Item>
					{/each}
				</Command.Group>
            </ScrollArea>
            </Command.List>
	</Command.Dialog>
</div>
