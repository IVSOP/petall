<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
	import '../../../app.css';
</script>

<div class="flex min-h-svh flex-col items-center justify-center gap-6 bg-muted p-6 md:p-10">
	<div class="flex w-full max-w-sm flex-col gap-6">
		<Card.Root>
			<Card.Header class="text-center">
				<Card.Title class="text-xl">Authenticating...</Card.Title>
				<Card.Description>Please wait while we complete your sign in</Card.Description>
			</Card.Header>
			<Card.Content class="flex justify-center py-8">
				<div
					class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"
				></div>
			</Card.Content>
		</Card.Root>
	</div>
</div>
