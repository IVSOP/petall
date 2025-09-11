<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';

	const { community } = $props();
</script>

<a
	href={`/community/${community.id}`}
	class="block transform rounded-md transition-transform duration-300 hover:scale-102 hover:shadow-lg"
>
	<Card.Root>
		<Card.Header>
			<Card.Title>{community.entity}</Card.Title>
			<Card.Description>Energy Comunity</Card.Description>
		</Card.Header>
		<Card.Content>
			<p>ID: {community.id}</p>
			{#if community.supplier}
				<p>Fornecedor: {community.supplier.name || community.supplier}</p>
			{/if}
		</Card.Content>
		<Card.Footer>
			<p class="text-sm text-muted-foreground">
				Last Update: {new Date().toLocaleDateString()}
			</p>
		</Card.Footer>
	</Card.Root>
</a>
