<script lang="ts">
	import type { EnergyRecord } from '$lib';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import TrendingDown from '@lucide/svelte/icons/trending-down';
	import TrendingUp from '@lucide/svelte/icons/trending-up';
    import type { EnergyStats } from '$lib';

	// let { energyRecords: energyTransfers = [] }: { energyRecords?: EnergyRecord[] } = $props();

	// let prices = $derived(
	// 	energyTransfers.map((transfer) => {
	// 		let price_consumed = (transfer.consumed || 0) * (transfer.consumerPrice || 0);
	// 		let price_generated = (transfer.generated || 0) * (transfer.sellerPrice || 0);
	// 		return price_generated - price_consumed;
	// 	})
	// );

    interface $$Props {
        statsLast60: EnergyStats[];
        // Add other props here
    }

    let { statsLast60 = {} } = $props();

    // [0] is the most recent date, so [i] is i days ago. get [0] to [30]
    let price_last_30 = $derived.by(() => {
        let total = 0;
        for (let i = 0; i < statsLast60.length && i < 30; i++) {
            total += statsLast60[i].generatedPrice - statsLast60[i].consumedPrice;
        }
        return total;
    })

    // [0] is the most recent date, so [i] is i days ago. get [30] to [60]
    let price_last_60_to_last_30 = $derived.by(() => {
        let total = 0;
        for (let i = 30; i < statsLast60.length && i < 60; i++) {
            total += statsLast60[i].generatedPrice - statsLast60[i].consumedPrice;
        }
        return total - price_last_30;
    })

    let price_last_60_to_last_30_percent = $derived.by(() => {
        return ((price_last_30 - price_last_60_to_last_30) / Math.abs(price_last_60_to_last_30)) * 100
    })

    $effect(() => {
        // console.log(`total_price ${total_price} and has type ${typeof total_price}`);
        // console.log(`statsAll is ${JSON.stringify(statsAll)}`);
        // console.log(`stats last 30 ${JSON.stringify(statsLast60)}`)
        console.log(`price last 60 to 30 ${JSON.stringify(price_last_60_to_last_30)}`)
        console.log(`price last 30 ${JSON.stringify(price_last_30)}`)
    })

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
		<Card.Description>Last 30 days balance</Card.Description>
		<Card.Title class="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
			€{price_last_30.toFixed(2)}
		</Card.Title>

		<Card.Action>
			<Badge
				variant={price_last_60_to_last_30_percent >= 0 ? 'outline' : 'destructive'}
				class="flex items-center gap-1"
			>
				{#if price_last_60_to_last_30_percent >= 0}
					<TrendingUp class="size-4 text-green-500" />
					+{price_last_60_to_last_30_percent.toFixed(1)}%
				{:else}
					<TrendingDown class="size-4 text-red-500" />
					{price_last_60_to_last_30_percent.toFixed(1)}%
				{/if}
			</Badge>
		</Card.Action>
	</Card.Header>

	<Card.Footer class="flex-col items-start gap-1.5 text-sm">
		{#if price_last_60_to_last_30_percent >= 0}
			<div class="line-clamp-1 flex gap-2 font-medium">
				Trending up this month <TrendingUp class="size-4 text-green-500" />
			</div>
			<div class="text-muted-foreground">Performance improved</div>
		{:else}
			<div class="line-clamp-1 flex gap-2 font-medium">
				Trending down this month <TrendingDown class="size-4 text-red-500" />
			</div>
			<div class="text-muted-foreground">Performance decreased</div>
		{/if}
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
