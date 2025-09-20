<script>
	import CommunityCard from '../../components/CommunityCard.svelte';

	const { data } = $props();
</script>

<!--  {JSON.stringify(data.communities)} -->

<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
	{#each data.communities as community}
		<CommunityCard {community} />
	{:else}
		<p>Nenhuma comunidade encontrada.</p>
	{/each}
</div>
