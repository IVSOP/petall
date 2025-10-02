<script lang="ts">
	import CommunityCard from '$lib/components/CommunityCard.svelte';
	import CommunityDialog from '$lib/components/CommunityDialog.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import SmartSelect from '$lib/components/SmartSelect.svelte';

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
		data.communities.filter((c) =>
			c.community.name.toLowerCase().includes(filterCommunityName.toLowerCase())
		)
	);
</script>

<div class="flex flex-col gap-2 pb-4 md:flex-row md:items-center">
	<div class="order-2 w-full md:order-none">
		<SearchBar bind:value={filterCommunityName} placeholder="Find community..." />
	</div>
	<div class="order-3 flex flex-row gap-2 md:order-none">
		<SmartSelect type="multiple" label="Role" items={roleItems} />
		<SmartSelect type="single" label="Sort" items={sortItems} />
	</div>
	<div class="order-1 md:order-none">
		<CommunityDialog />
	</div>
</div>

<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
	{#if filteredCommunities.length > 0}
		{#each filteredCommunities as community}
			<CommunityCard {community} />
		{/each}
	{:else}
		<div class="col-span-full flex flex-col items-center justify-center py-12">
			<p class="mb-2 text-lg font-medium text-gray-700 sm:text-2xl">No community found</p>
		</div>
	{/if}
</div>
