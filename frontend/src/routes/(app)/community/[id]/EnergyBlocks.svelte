<script lang="ts">
	import type { EnergyRecord } from '$lib';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import TrendingDown from '@lucide/svelte/icons/trending-down';
	import TrendingUp from '@lucide/svelte/icons/trending-up';

	let { energyRecords: energyTransfers = [] }: { energyRecords?: EnergyRecord[] } = $props();

	let prices = $derived(
		energyTransfers.map((transfer) => {
			let price_consumed = (transfer.consumed || 0) * (transfer.consumerPrice || 0);
			let price_generated = (transfer.generated || 0) * (transfer.sellerPrice || 0);
			return price_generated - price_consumed;
		})
	);

	let total_price = $derived.by(() => {
		return prices.reduce((total, price) => total + (isNaN(price) ? 0 : price), 0);
	});

	//   $effect(() => {
	//     console.log('Prices:', prices);
	//     console.log('Total Price:', total_price);
	//   });
</script>

<div
	class="grid grid-cols-1 gap-4 px-0 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card *:data-[slot=card]:shadow-xs lg:px-0 @xl/main:grid-cols-2 @5xl/main:grid-cols-4 dark:*:data-[slot=card]:bg-card"
>
	<Card.Root class="@container/card">
		<Card.Header>
			<Card.Description>Total Balance</Card.Description>
			<Card.Title class="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
				€{total_price.toFixed(2)}
			</Card.Title>
			<Card.Action>
				<Badge variant="outline">
					<TrendingUp />
					+12.5%
				</Badge>
			</Card.Action>
		</Card.Header>
		<Card.Footer class="flex-col items-start gap-1.5 text-sm">
			<div class="line-clamp-1 flex gap-2 font-medium">
				Trending up this month <TrendingUp class="size-4" />
			</div>
			<div class="text-muted-foreground">Visitors for the last 6 months</div>
		</Card.Footer>
	</Card.Root>
	<Card.Root class="@container/card">
		<Card.Header>
			<Card.Description>New Customers</Card.Description>
			<Card.Title class="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
				1,234
			</Card.Title>
			<Card.Action>
				<Badge variant="outline">
					<TrendingDown />
					-20%
				</Badge>
			</Card.Action>
		</Card.Header>
		<Card.Footer class="flex-col items-start gap-1.5 text-sm">
			<div class="line-clamp-1 flex gap-2 font-medium">
				Down 20% this period <TrendingDown class="size-4" />
			</div>
			<div class="text-muted-foreground">Acquisition needs attention</div>
		</Card.Footer>
	</Card.Root>
	<Card.Root class="@container/card">
		<Card.Header>
			<Card.Description>Active Accounts</Card.Description>
			<Card.Title class="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
				45,678
			</Card.Title>
			<Card.Action>
				<Badge variant="outline">
					<TrendingUp />
					+12.5%
				</Badge>
			</Card.Action>
		</Card.Header>
		<Card.Footer class="flex-col items-start gap-1.5 text-sm">
			<div class="line-clamp-1 flex gap-2 font-medium">
				Strong user retention <TrendingUp class="size-4" />
			</div>
			<div class="text-muted-foreground">Engagement exceed targets</div>
		</Card.Footer>
	</Card.Root>
	<Card.Root class="@container/card">
		<Card.Header>
			<Card.Description>Growth Rate</Card.Description>
			<Card.Title class="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
				4.5%
			</Card.Title>
			<Card.Action>
				<Badge variant="outline">
					<TrendingUp />
					+4.5%
				</Badge>
			</Card.Action>
		</Card.Header>
		<Card.Footer class="flex-col items-start gap-1.5 text-sm">
			<div class="line-clamp-1 flex gap-2 font-medium">
				Steady performance increase <TrendingUp class="size-4" />
			</div>
			<div class="text-muted-foreground">Meets growth projections</div>
		</Card.Footer>
	</Card.Root>
</div>
