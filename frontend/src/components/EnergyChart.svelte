<script lang="ts">
	import TrendingUpIcon from '@lucide/svelte/icons/trending-up';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import { scaleUtc } from 'd3-scale';
	import { Area, AreaChart, ChartClipPath } from 'layerchart';
	import { curveNatural } from 'd3-shape';
	import ChartContainer from '$lib/components/ui/chart/chart-container.svelte';

	import { cubicInOut } from 'svelte/easing';

	let { energyTransfers = [], participant_id = '' } = $props();

	let timeRange = $state('30d');

	const selectedLabel = $derived.by(() => {
		switch (timeRange) {
			case '30d':
				return '30 dias';
			case '7d':
				return '7 dias';
			case '1d':
				return '24 horas';
			default:
				return '30 dias';
		}
	});

    const xAxisFormat = $derived.by(() => {
        return (v: Date) => {
            switch (timeRange) {
                case '1d':
                return v.toLocaleTimeString('en-US', {
                    hour: '2-digit',
                    minute: '2-digit',
                    hour12: false
                });
                default:
                return v.toLocaleDateString('en-US', {
                    month: 'short',
                    day: 'numeric'
                });
            }
        };
    });

    const ticksFunction = $derived.by(() => {
        switch (timeRange) {
            case '30d':
                return 30; // 1 per day
            case '7d':
                return 7; // 1 per day
            case '1d':
                return 24 * 2; // every 30 mins
            default:
                return undefined
        };
    });

	const filteredData = $derived(
		energyTransfers
            // TODO: filter so that 30d shows only results from the last 30 days, etc
			// .filter((item) => {
			// 	const date = new Date(item.start);
			// 	const now = new Date();
			// 	if (timeRange === '30d') {
			// 		const thirtyDaysAgo = new Date(now.getTime() - 30 * 24 * 60 * 60 * 1000);
			// 		return date >= thirtyDaysAgo && date <= now;
			// 	}
			// 	return true; // Include all data for '7d' and '1d'
			// })
			.map((item) => ({
				date: new Date(item.start),
				generated: item.generated,
				consumed: item.consumed
			}))
	);

	const chartConfig = {
		generated: { label: 'Generated', color: 'var(--chart-1)' },
		consumed: { label: 'Consumed', color: 'var(--chart-2)' }
	} satisfies Chart.ChartConfig;
</script>

<Card.Root>
	<Card.Header class="flex items-center gap-2 space-y-0 border-b py-5 sm:flex-row">
		<div class="grid flex-1 gap-1 text-center sm:text-left">
			<Card.Title>Area Chart - Interactive</Card.Title>
			<Card.Description>Showing total visitors for the last 3 months</Card.Description>
		</div>
		<Select.Root type="single" bind:value={timeRange}>
			<Select.Trigger class="w-[160px] rounded-lg sm:ml-auto" aria-label="Select a value">
				{selectedLabel}
			</Select.Trigger>
			<Select.Content class="rounded-xl">
				<Select.Item value="30d" class="rounded-lg">30 dias</Select.Item>
				<Select.Item value="7d" class="rounded-lg">7 dias</Select.Item>
				<Select.Item value="1d" class="rounded-lg">24 horas</Select.Item>
			</Select.Content>
		</Select.Root>
	</Card.Header>
	<Card.Content>
		<ChartContainer config={chartConfig} class="aspect-auto h-[250px] w-full">
			<AreaChart
				legend
				data={filteredData}
				x="date"
				xScale={scaleUtc()}
				series={[
					{
						key: 'consumed',
						label: 'Consumed',
						color: chartConfig.consumed.color
					},
					{
						key: 'generated',
						label: 'Generated',
						color: chartConfig.generated.color
					}
				]}
				seriesLayout="stack"
				props={{
					area: {
						curve: curveNatural,
						'fill-opacity': 0.4,
						line: { class: 'stroke-1' },
						motion: 'tween'
					},
					xAxis: {
                        ticks: ticksFunction,
                        format: xAxisFormat
					},

					yAxis: { format: () => '' }
				}}
			>
				{#snippet marks({ series, getAreaProps })}
					<defs>
						<linearGradient id="fillGenerated" x1="0" y1="0" x2="0" y2="1">
							<stop offset="5%" stop-color="var(--color-generated)" stop-opacity={1.0} />
							<stop offset="95%" stop-color="var(--color-generated)" stop-opacity={0.1} />
						</linearGradient>
						<linearGradient id="fillConsumed" x1="0" y1="0" x2="0" y2="1">
							<stop offset="5%" stop-color="var(--color-consumed)" stop-opacity={0.8} />
							<stop offset="95%" stop-color="var(--color-consumed)" stop-opacity={0.1} />
						</linearGradient>
					</defs>
					<ChartClipPath
						initialWidth={0}
						motion={{
							width: { type: 'tween', duration: 1000, easing: cubicInOut }
						}}
					>
						{#each series as s, i (s.key)}
							<Area
								{...getAreaProps(s, i)}
								fill={s.key === 'generated' ? 'url(#fillGenerated)' : 'url(#fillConsumed)'}
							/>
						{/each}
					</ChartClipPath>
				{/snippet}
				{#snippet tooltip()}
					<Chart.Tooltip
						labelFormatter={(v: Date) => {
							return v.toLocaleDateString('en-US', {
								month: 'long'
							});
						}}
						indicator="line"
					/>
				{/snippet}
			</AreaChart>
		</ChartContainer>
	</Card.Content>
	<Card.Footer>
		<div class="flex w-full items-start gap-2 text-sm">
			<div class="grid gap-2">
				<div class="flex items-center gap-2 leading-none font-medium">
					Trending up by 5.2% this month <TrendingUpIcon class="size-4" />
				</div>
				<div class="flex items-center gap-2 leading-none text-muted-foreground">
					January - June 2024
				</div>
			</div>
		</div>
	</Card.Footer>
</Card.Root>
