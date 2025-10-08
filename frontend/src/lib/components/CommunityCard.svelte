<script lang="ts">
	import Users from '@lucide/svelte/icons/users';
	import Zap from '@lucide/svelte/icons/zap';
	import Activity from '@lucide/svelte/icons/activity';
	import Leaf from '@lucide/svelte/icons/leaf';
	import DollarSign from '@lucide/svelte/icons/dollar-sign';
	import TrendingDown from '@lucide/svelte/icons/trending-down';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { Progress } from '$lib/components/ui/progress/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import type { CommunityGetCommunitiesResponse } from '$lib/api/community';
	import { onMount } from 'svelte';

	const { community }: { community: CommunityGetCommunitiesResponse } = $props();

	let value = $state(0);

	onMount(() => {
		const timer = setTimeout(() => ((value = Math.floor(Math.random() * 100)), 100));
		return () => clearTimeout(timer);
	});

	console.log(community);
</script>

<a href={`/community/${community.id}`}>
	<Card.Root
		class="w-full gap-4 border-border/50 shadow-lg transition-all duration-300 hover:scale-[1.01] hover:border-primary/20 hover:shadow-xl "
	>
		<Card.Header>
			<div class="flex flex-col gap-4 sm:flex-row sm:items-start">
				<div
					class="flex h-16 w-16 shrink-0 items-center justify-center rounded-2xl bg-gradient-to-br from-primary/20 to-primary/5 ring-1 ring-primary/10"
				>
					<Zap class="h-8 w-8 text-primary" />
				</div>

				<div class="flex-1 space-y-2">
					<div class="flex flex-wrap items-center gap-3">
						<h3 class="text-3xl font-bold text-balance">{community.name}</h3>
						<div
							class="flex items-center gap-1.5 rounded-full bg-emerald-500/10 px-3 py-1 ring-1 ring-emerald-500/20"
						>
							<Activity class="h-3.5 w-3.5 text-emerald-600 dark:text-emerald-400" />
							<span class="text-xs font-semibold text-emerald-600 dark:text-emerald-400">
								Active
							</span>
						</div>
					</div>

					<p class="max-w-2xl text-sm leading-relaxed text-muted-foreground">
						{community.description}
					</p>

					<div class="flex flex-wrap gap-2 sm:hidden">
						<Badge variant="secondary" class="text-sm">User</Badge>
						<Badge variant="secondary" class="text-sm">Manager</Badge>
					</div>
				</div>

				<div class="hidden gap-2 pt-1 sm:flex">
					<Badge variant="secondary" class="text-sm">User</Badge>
					<Badge variant="secondary" class="text-sm">Manager</Badge>
				</div>
			</div>
		</Card.Header>

		<Card.Content class="space-y-6">
			<div class="grid grid-cols-2 gap-6 sm:grid-cols-3 lg:grid-cols-5">
				<div class="space-y-2 rounded-xl bg-muted/30 p-4 ring-1 ring-border/50">
					<div class="flex items-center gap-2 text-muted-foreground">
						<div class="rounded-lg bg-blue-500/10 p-1.5">
							<Users class="h-4 w-4 text-blue-600 dark:text-blue-400" />
						</div>
						<span class="text-xs font-medium">Participants</span>
					</div>
					<p class="text-4xl font-bold tabular-nums">{15}</p>
					<p class="text-xs text-muted-foreground">{10} active</p>
				</div>

				<div class="space-y-2 rounded-xl bg-muted/30 p-4 ring-1 ring-border/50">
					<div class="flex items-center gap-2 text-muted-foreground">
						<div class="rounded-lg bg-yellow-500/10 p-1.5">
							<Zap class="h-4 w-4 text-yellow-600 dark:text-yellow-400" />
						</div>
						<span class="text-xs font-medium">Produced</span>
					</div>
					<p class="text-4xl font-bold tabular-nums">
						{(Math.floor(Math.random() * 10000) / 100).toFixed(1)}
						<span class="ml-1 text-base font-normal text-muted-foreground">MWh</span>
					</p>
					<p class="text-xs text-muted-foreground">
						{(Math.floor(Math.random() * 10000) / 100).toFixed(1)} kWh used
					</p>
				</div>

				<div class="space-y-2 rounded-xl bg-muted/30 p-4 ring-1 ring-border/50">
					<div class="flex items-center gap-2 text-muted-foreground">
						<div class="rounded-lg bg-emerald-500/10 p-1.5">
							<Leaf class="h-4 w-4 text-emerald-600 dark:text-emerald-400" />
						</div>
						<span class="text-xs font-medium">CO₂ Saved</span>
					</div>
					<p class="text-4xl font-bold tabular-nums">
						{(Math.floor(Math.random() * 10000) / 100).toFixed(1)}
						<span class="ml-1 text-base font-normal text-muted-foreground">t</span>
					</p>
					<p class="text-xs text-muted-foreground">this month</p>
				</div>

				<div class="space-y-2 rounded-xl bg-muted/30 p-4 ring-1 ring-border/50">
					<div class="flex items-center gap-2 text-muted-foreground">
						<div class="rounded-lg bg-green-500/10 p-1.5">
							<DollarSign class="h-4 w-4 text-green-600 dark:text-green-400" />
						</div>
						<span class="text-xs font-medium">Savings</span>
					</div>
					<p class="text-4xl font-bold tabular-nums">
						${(Math.floor(Math.random() * 10000) / 100).toFixed(1)}
						<span class="ml-1 text-base font-normal text-muted-foreground">k</span>
					</p>
					<p class="text-xs text-muted-foreground">total saved</p>
				</div>

				<div class="space-y-2 rounded-xl bg-muted/30 p-4 ring-1 ring-border/50">
					<div class="flex items-center gap-2 text-muted-foreground">
						<div class="rounded-lg bg-cyan-500/10 p-1.5">
							<TrendingDown class="h-4 w-4 text-cyan-600 dark:text-cyan-400" />
						</div>
						<span class="text-xs font-medium">Efficiency</span>
					</div>
					<p class="text-4xl font-bold tabular-nums">
						{(Math.floor(Math.random() * 10000) / 100).toFixed(1)}
						<span class="ml-1 text-base font-normal text-muted-foreground">%</span>
					</p>
					<p class="text-xs text-muted-foreground">surplus rate</p>
				</div>
			</div>

			<div class="space-y-3 rounded-xl bg-muted/30 p-4 ring-1 ring-border/50">
				<div class="flex items-center justify-between text-sm">
					<span class="font-medium text-foreground">Monthly Energy Goal</span>
					<span class="font-bold text-primary">{value}% achieved</span>
				</div>
				<Progress {value} max={100} />
			</div>
		</Card.Content>
	</Card.Root>
</a>
