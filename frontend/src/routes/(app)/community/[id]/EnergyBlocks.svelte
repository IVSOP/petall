<script lang="ts">
	import type { EnergyRecord } from '$lib';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import TrendingDown from '@lucide/svelte/icons/trending-down';
	import TrendingUp from '@lucide/svelte/icons/trending-up';
    import type { EnergyStats } from '$lib';

    interface $$Props {
        statsLast60: EnergyStats[];
        // Add other props here
    }

    let { statsLast60 = {} } = $props();


    //////////////////////// eu sei que isto está cagado, foi só para ser rápido até saber o que ia ficar melhor. depois arranjos

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
        return total;
    })

    let price_last_60_to_last_30_percent = $derived.by(() => {
        return ((price_last_30 - price_last_60_to_last_30) / Math.abs(price_last_60_to_last_30)) * 100
    })

    let delta_last_30 = $derived.by(() => {
        let total = 0;
        for (let i = 0; i < statsLast60.length && i < 30; i++) {
            total += statsLast60[i].generatedSum - statsLast60[i].consumedSum;
        }
        return total;
    })
    let delta_last_60_to_last_30 = $derived.by(() => {
        let total = 0;
        for (let i = 30; i < statsLast60.length && i < 60; i++) {
            total += statsLast60[i].generatedSum - statsLast60[i].consumedSum;
        }
        return total;
    })
    let delta_last_60_to_last_30_percent = $derived.by(() => {
        return ((delta_last_30 - delta_last_60_to_last_30) / Math.abs(delta_last_60_to_last_30)) * 100
    })

    let generated_last_30 = $derived.by(() => {
        let total = 0;
        for (let i = 0; i < statsLast60.length && i < 30; i++) {
            total += statsLast60[i].generatedSum;
        }
        return total;
    })
    let generated_last_60_to_last_30 = $derived.by(() => {
        let total = 0;
        for (let i = 30; i < statsLast60.length && i < 60; i++) {
            total += statsLast60[i].generatedSum;
        }
        return total;
    })
    let generated_last_60_to_last_30_percent = $derived.by(() => {
        return ((generated_last_30 - generated_last_60_to_last_30) / Math.abs(generated_last_60_to_last_30)) * 100
    })

    let consumed_last_30 = $derived.by(() => {
        let total = 0;
        for (let i = 0; i < statsLast60.length && i < 30; i++) {
            total += statsLast60[i].consumedSum;
        }
        return total;
    })
    let consumed_last_60_to_last_30 = $derived.by(() => {
        let total = 0;
        for (let i = 30; i < statsLast60.length && i < 60; i++) {
            total += statsLast60[i].consumedSum;
        }
        return total;
    })
    let consumed_last_60_to_last_30_percent = $derived.by(() => {
        return ((consumed_last_30 - consumed_last_60_to_last_30) / Math.abs(consumed_last_60_to_last_30)) * 100
    })

    $effect(() => {
        // console.log(`total_price ${total_price} and has type ${typeof total_price}`);
        // console.log(`statsAll is ${JSON.stringify(statsAll)}`);
        // console.log(`stats last 30 ${JSON.stringify(statsLast60)}`)
        console.log(`price last 60 to 30 ${JSON.stringify(price_last_60_to_last_30)}`)
        console.log(`price last 30 ${JSON.stringify(price_last_30)}`)
        console.log(`delta last 60 to 30 ${JSON.stringify(delta_last_60_to_last_30)}`)
        console.log(`delta last 30 ${JSON.stringify(delta_last_30)}`)
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
                    variant='outline'
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
			<Card.Description>Last 30 days energy balance</Card.Description>
			<Card.Title class="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
				{(delta_last_30 / 1000).toFixed(2)}kWh
			</Card.Title>

			<Card.Action>
                <Badge
                    variant='outline'
                    class="flex items-center gap-1"
                >
                    {#if delta_last_60_to_last_30_percent >= 0}
                        <TrendingUp class="size-4 text-green-500" />
                        +{delta_last_60_to_last_30_percent.toFixed(1)}%
                    {:else}
                        <TrendingDown class="size-4 text-red-500" />
                        {delta_last_60_to_last_30_percent.toFixed(1)}%
                    {/if}
				</Badge>
			</Card.Action>
        </Card.Header>

        <Card.Footer class="flex-col items-start gap-1.5 text-sm">
            {#if price_last_60_to_last_30_percent >= 0}
                <div class="line-clamp-1 flex gap-2 font-medium">
                    Trending up this month <TrendingUp class="size-4 text-green-500" />
                </div>
            {:else}
                <div class="line-clamp-1 flex gap-2 font-medium">
                    Trending down this month <TrendingDown class="size-4 text-red-500" />
                </div>
                {/if}
            {#if price_last_30 >= 0}
                <div class="text-muted-foreground">Balance &gt; 0: More energy provided than consumed</div>
            {:else}
                <div class="text-muted-foreground">Balance &lt; 0: More energy consumed than provided</div>
            {/if}
        </Card.Footer>
	</Card.Root>

    <Card.Root class="@container/card">
        <Card.Header>
            <Card.Description>Last 30 days energy generated</Card.Description>
            <Card.Title class="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
                {(generated_last_30 / 1000).toFixed(2)}kWh
            </Card.Title>

            <Card.Action>
                <Badge
                    variant='outline'
                    class="flex items-center gap-1"
                >
                    {#if generated_last_60_to_last_30_percent >= 0}
                        <TrendingUp class="size-4 text-green-500" />
                        +{generated_last_60_to_last_30_percent.toFixed(1)}%
                    {:else}
                        <TrendingDown class="size-4 text-red-500" />
                        {generated_last_60_to_last_30_percent.toFixed(1)}%
                    {/if}
                </Badge>
            </Card.Action>
        </Card.Header>

        <Card.Footer class="flex-col items-start gap-1.5 text-sm">
            {#if generated_last_60_to_last_30_percent >= 0}
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
            <Card.Description>Last 30 days energy consumed</Card.Description>
            <Card.Title class="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
                {(consumed_last_30 / 1000).toFixed(2)}kWh
            </Card.Title>

            <Card.Action>
                <Badge
                    variant='outline'
                    class="flex items-center gap-1"
                >
                    {#if consumed_last_60_to_last_30_percent >= 0}
                        <TrendingUp class="size-4 text-red-500" />
                        +{consumed_last_60_to_last_30_percent.toFixed(1)}%
                    {:else}
                        <TrendingDown class="size-4 text-green-500" />
                        {consumed_last_60_to_last_30_percent.toFixed(1)}%
                    {/if}
                </Badge>
            </Card.Action>
        </Card.Header>

        <Card.Footer class="flex-col items-start gap-1.5 text-sm">
            {#if consumed_last_60_to_last_30_percent >= 0}
                <div class="line-clamp-1 flex gap-2 font-medium">
                    Trending up this month <TrendingUp class="size-4 text-red-500" />
                </div>
                <div class="text-muted-foreground">Performance decreased</div>
            {:else}
                <div class="line-clamp-1 flex gap-2 font-medium">
                    Trending down this month <TrendingDown class="size-4 text-green-500" />
                </div>
                <div class="text-muted-foreground">Performance improved</div>
            {/if}
        </Card.Footer>
    </Card.Root>
</div>
