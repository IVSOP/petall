<script lang="ts">
	import type { Participant } from '$lib';
	import SearchBar from './SearchBar.svelte';
	import SquarePlus from '@lucide/svelte/icons/square-plus';
	import { Button } from '$lib/components/ui/button/index.js';
	import ParticipantSearchDialog from './ParticipantSearchDialog.svelte';
	import ParticipantScrollList from './ParticipantScrollList.svelte';

	let { participants }: { participants: Participant[] } = $props();

	let open = $state(false);
	let filterParticipantName = $state('');

	let filterParticipants = $derived(
		participants.filter((p) => p.name.toLowerCase().includes(filterParticipantName.toLowerCase()))
	);

	let roleItems = [
		{ value: 'user', label: 'User' },
		{ value: 'manager', label: 'Manager' }
	];
</script>

<div class="grid gap-4">
	<div class="text-w-full rounded-md border bg-muted px-4 py-2 text-lg font-semibold md:text-xl">
		Manage access
	</div>

	<div class="flex flex-col gap-2 md:flex-row md:items-center">
		<div class="order-2 w-full md:order-none">
			<SearchBar bind:value={filterParticipantName} placeholder="Find participant..." />
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

	<ParticipantScrollList participants={filterParticipants} {roleItems} />
	<ParticipantSearchDialog bind:open {participants} />
</div>
