<script lang="ts">
	const { data } = $props();
</script>

Manage communities (showing communities that you have access):

{#each data.communities as community}
	<p>{JSON.stringify(community)}</p>
{/each}
