<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';

	const { community } = $props();
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>{community.entity}</Card.Title>
		<Card.Description>Comunidade energética</Card.Description>
	</Card.Header>
	<Card.Content>
		<p>ID: {community.id}</p>
		{#if community.supplier}
			<p>Fornecedor: {community.supplier.name || community.supplier}</p>
		{/if}
	</Card.Content>
	<Card.Footer>
		<p class="text-sm text-muted-foreground">
            <!-- por a hora da ultima transferencia?-->
			Última atualização: {new Date().toLocaleDateString()}
		</p>
	</Card.Footer>
</Card.Root>

