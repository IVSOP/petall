<script lang="ts">
	import CommunityCard from '$lib/components/CommunityCard.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import SmartSelect from '$lib/components/SmartSelect.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import SquarePlus from '@lucide/svelte/icons/square-plus';

	const { data } = $props();

	let filterCommunityName = $state('');

	let roleItems = [
		{ value: 'user', label: 'User' },
		{ value: 'manager', label: 'Manager' }
	];

	let sortItems = [
		{ value: 'byDate', label: 'Date' },
		{ value: 'byName', label: 'Name' },
		{ value: 'byParticipants', label: 'Participants' }
	];

	let filteredCommunities = $derived(
		data.communities.filter((c) => c.name.toLowerCase().includes(filterCommunityName.toLowerCase()))
	);
</script>

<div class="flex flex-col gap-2 pb-4 md:flex-row md:items-center">
	<div class="order-2 w-full md:order-none">
		<SearchBar bind:filter={filterCommunityName} placeholder="Find community..." />
	</div>
	<div class="order-3 flex flex-row gap-2 md:order-none">
		<SmartSelect type="multiple" label="Role" items={roleItems} />
		<SmartSelect type="single" label="Sort" items={sortItems} />
	</div>
	<div class="order-1 md:order-none">
		<Button
			variant="outline"
			class={`w-full bg-green-600 text-sm text-white hover:bg-green-700 hover:text-white focus:outline-none md:text-base`}
			href="/community/new"
		>
			<SquarePlus />
			New Community
		</Button>
	</div>
</div>

<div class="grid grid-cols-1 gap-4">
	{#if filteredCommunities.length > 0}
		{#each filteredCommunities as community}
			<CommunityCard {community} />
		{/each}
	{:else}
		<div class="col-span-full flex flex-col items-center justify-center py-12">
			<p class="mb-2 text-lg font-medium text-gray-700 sm:text-2xl">
				You are not a member of any community
			</p>
		</div>
	{/if}
</div>
