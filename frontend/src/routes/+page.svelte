<script>
	import CommunityCard from '../components/CommunityCard.svelte';

	const { data } = $props();
</script>

{JSON.stringify(data.communities)}

<div class="grid grid-cols-4 gap-4">
	<CommunityCard />
</div>
