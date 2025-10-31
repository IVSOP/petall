<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import HousePlug from '@lucide/svelte/icons/house-plug';
	import Settings from '@lucide/svelte/icons/settings';
	import Zap from '@lucide/svelte/icons/zap';
	import Users from '@lucide/svelte/icons/users';
	import HousePlus from '@lucide/svelte/icons/house-plus';
	import Badge from '$lib/components/ui/badge/badge.svelte';

	const { data } = $props();
</script>

<svelte:head>
	<title>Admin View</title>
</svelte:head>

<Card.Root class="w-full gap-4">
	<Card.Header class="flex items-center justify-between border-b">
		<div class="flex items-center gap-2">
			<HousePlug />
			<h2 class="text-lg font-semibold">Energy Communities</h2>
			<span class="text-sm text-muted-foreground">
				{data.communities.length}
			</span>
		</div>
		{#if data.user?.isAdmin}
			<Button href="/admin/community/new" class="cursor-pointer">
				<HousePlus />
				Create community
			</Button>
		{/if}
	</Card.Header>

	<Card.Content>
		{#if data.communities.length > 0}
			<div class="space-y-3">
				{#each data.communities as community}
					<div
						class="group relative overflow-hidden rounded-xl border bg-card p-4 transition-colors hover:bg-accent/50"
					>
						<div class="relative flex items-center justify-between gap-4">
							<div class="flex min-w-0 flex-1 items-center gap-4">
								<Avatar.Root class="h-10 w-10">
									<Avatar.Fallback class="bg-primary/10 text-sm font-medium">
										<Zap />
									</Avatar.Fallback>
								</Avatar.Root>

								<div class="flex min-w-0 flex-1 flex-col gap-1">
									<h3 class="truncate text-base font-semibold">
										{community.name}
									</h3>
									<Badge variant="secondary" class="w-fit gap-1.5 px-2 py-0.5">
										<Users />
										<span class="text-xs font-medium">
											{Number(community.userCount).toLocaleString()}
											{Number(community.userCount) === 1 ? 'user' : 'users'}
										</span>
									</Badge>
								</div>
							</div>

							<Button href="/admin/community/{community.id}/settings">
								<Settings /> Settings
							</Button>
						</div>
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
