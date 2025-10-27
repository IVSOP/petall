<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import HousePlug from '@lucide/svelte/icons/house-plug';
	import Settings from '@lucide/svelte/icons/settings';
	import Zap from '@lucide/svelte/icons/zap';

	const { data } = $props();
</script>

<Card.Root class="w-full gap-4">
	<Card.Header class="flex items-center justify-between border-b">
		<div class="flex items-center gap-2">
			<HousePlug />
			<h2 class="text-lg font-semibold">Energy Communities</h2>
			<span class="text-sm text-muted-foreground">
				{data.communities.length}
			</span>
		</div>
	</Card.Header>

	<Card.Content>
		{#if data.communities.length > 0}
			<div class="space-y-3">
				{#each data.communities as community}
					<div
						class="flex items-center justify-between gap-3 rounded-lg border bg-card p-3 transition-colors hover:bg-accent/50"
					>
						<div class="flex items-center gap-3">
							<Avatar.Root class="h-10 w-10">
								<Avatar.Fallback class="bg-primary/10 text-sm font-medium">
									<Zap />
								</Avatar.Fallback>
							</Avatar.Root>
							<span class="font-medium">{community.name}</span>
						</div>

						<Button
							href="/community/{community.id}/settings"
							class="cursor-pointer text-sm font-medium"
						>
							<Settings />
							Settings
						</Button>
					</div>
				{/each}
			</div>
		{:else}
			<div
				class="flex flex-col items-center justify-center gap-2 pt-2 text-center text-muted-foreground"
			>
				<Zap class="h-8 w-8 text-gray-400" />
				<p class="text-lg font-medium">No energy communities found.</p>
			</div>
		{/if}
	</Card.Content>
</Card.Root>
