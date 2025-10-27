<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import HousePlug from '@lucide/svelte/icons/house-plug';
	import Settings from '@lucide/svelte/icons/settings';
	import Zap from '@lucide/svelte/icons/zap';
	
	const { data } = $props();
</script>

<div class="container">
	<div class="flex flex-1 flex-col">
		<div class="@container/main flex flex-1 flex-col gap-2">
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
				</Card.Content>
			</Card.Root>
		</div>
	</div>
</div>
