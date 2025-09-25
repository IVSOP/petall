<script lang="ts">
	import CommunityCard from '../../components/CommunityCard.svelte';
	import CommunityFilter from '../../components/CommunityFilter.svelte';

	const { data } = $props();

	let filterValue = $state('');

	let filteredCommunities = $derived(
		data.communities.filter((c) =>
			c.community.name.toLowerCase().includes(filterValue.toLowerCase())
		)
	);
</script>

<CommunityFilter bind:value={filterValue} />
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
