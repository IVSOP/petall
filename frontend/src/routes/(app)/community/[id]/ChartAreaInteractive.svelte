<script lang="ts">
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import { scaleUtc } from 'd3-scale';
	import { Area, AreaChart } from 'layerchart';
	import { curveNatural } from 'd3-shape';
    import type { EnergyStats } from '$lib';

    interface $$Props {
		statsLast60: EnergyStats[];
		// Add other props here
	}

	let { statsLast60 = {} } = $props();

	// [0] is the most recent date, so [i] is i days ago. get [0] to [30]
    let statsLast30: EnergyStats[] = $derived.by(() => {
        const res = [];
        let len = Math.min(statsLast60.length, 30);
        for (let i = 0; i < len; i++) {
            res.push(statsLast60[i]);
        }
        return res;
    });
    let statsLast7: EnergyStats[] = $derived.by(() => {
        const res = [];
        let len = Math.min(statsLast60.length, 7);
        for (let i = 0; i < len; i++) {
            res.push(statsLast60[i]);
        }
        return res;
    });

	let timeRange = $state('60d');
	const selectedLabel = $derived.by(() => {
		switch (timeRange) {
			case '60d':
				return 'Last 60 days';
			case '30d':
				return 'Last 30 days';
			case '7d':
				return 'Last 7 days';
			default:
				return 'Last 60 days';
		}
	});

	const filteredData = $derived.by(() => {
        switch (timeRange) {
			case '60d':
				return statsLast60;
			case '30d':
				return statsLast30;
			case '7d':
				return statsLast7;
			default:
				return statsLast60;
		}
    });

    $effect(() => {
        console.log(JSON.stringify(filteredData))
    })

	const chartConfig = {
		generated: { label: 'Generated kWh', color: '#3b82f6' }, // blue-500
		consumed: { label: 'Consumed kWh', color: '#ef4444' } // red-500
	} satisfies Chart.ChartConfig;
</script>

<style>
	.custom-legend {
		display: flex;
		gap: 1rem;
		align-items: center;
		margin-top: 0.5rem;
		font-size: 0.875rem;
	}
	.custom-legend-item {
		display: flex;
		align-items: center;
		gap: 0.4rem;
	}
	.legend-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
	}
</style>

<Card.Root class="@container/card">
	<Card.Header>
		<Card.Title>Total Visitors</Card.Title>
		<Card.Description>
			<span class="hidden @[540px]/card:block"> Total for the last 60 days </span>
			<span class="@[540px]/card:hidden">Last 60 days</span>
		</Card.Description>
		<Card.Action>
			<ToggleGroup.Root
				type="single"
				bind:value={timeRange}
				variant="outline"
				class="hidden *:data-[slot=toggle-group-item]:!px-4 @[767px]/card:flex"
			>
				<ToggleGroup.Item value="60d">Last 60 days</ToggleGroup.Item>
				<ToggleGroup.Item value="30d">Last 30 days</ToggleGroup.Item>
				<ToggleGroup.Item value="7d">Last 7 days</ToggleGroup.Item>
			</ToggleGroup.Root>
			<Select.Root type="single" bind:value={timeRange}>
				<Select.Trigger
					size="sm"
					class="flex w-40 **:data-[slot=select-value]:block **:data-[slot=select-value]:truncate @[767px]/card:hidden"
					aria-label="Select a value"
				>
					<span data-slot="select-value">
						{selectedLabel}
					</span>
				</Select.Trigger>
				<Select.Content class="rounded-xl">
					<Select.Item value="60d" class="rounded-lg">Last 60 days</Select.Item>
					<Select.Item value="30d" class="rounded-lg">Last 30 days</Select.Item>
					<Select.Item value="7d" class="rounded-lg">Last 7 days</Select.Item>
				</Select.Content>
			</Select.Root>
		</Card.Action>
	</Card.Header>
	<Card.Content class="px-2 pt-4 sm:px-6 sm:pt-6">
		<Chart.Container config={chartConfig} class="aspect-auto h-[250px] w-full">
            <!-- legend removed from here, manually implemented below -->
			<AreaChart
				data={filteredData}
				x="periodStart"
				xScale={scaleUtc()}
				series={[
					{
						key: 'generatedSum',
						label: 'Generated kWh',
						color: chartConfig.generated.color
					},
					{
						key: 'consumedSum',
						label: 'Consumed kWh',
						color: chartConfig.consumed.color
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
                        // WARN: when in 7d mode, force it to use 7 ticks
						ticks: timeRange === '7d' ? 7 : undefined,
						format: (v) => {
							return v.toLocaleDateString('en-US', {
								month: 'short',
								day: 'numeric'
							});
						}
					},
					yAxis: { format: () => '' }
				}}
			>
				{#snippet marks({ series, getAreaProps })}
					<defs>
						<linearGradient id="fillGenerated" x1="0" y1="0" x2="0" y2="1">
							<stop offset="5%" stop-color={chartConfig.generated.color} stop-opacity="0.9" />
							<stop offset="95%" stop-color={chartConfig.generated.color} stop-opacity="0.1" />
						</linearGradient>
						<linearGradient id="fillConsumed" x1="0" y1="0" x2="0" y2="1">
							<stop offset="5%" stop-color={chartConfig.consumed.color} stop-opacity="0.9" />
							<stop offset="95%" stop-color={chartConfig.consumed.color} stop-opacity="0.1" />
						</linearGradient>
					</defs>

					{#each series as s, i (s.key)}
						<Area
							{...getAreaProps(s, i)}
							fill={s.key === 'generatedSum'
								? 'url(#fillGenerated)'
								: 'url(#fillConsumed)'}
							stroke={s.key === 'generatedSum'
								? chartConfig.generated.color
								: chartConfig.consumed.color}
						/>
					{/each}
				{/snippet}
				{#snippet tooltip()}
					<Chart.Tooltip
						labelFormatter={(v: Date) => {
							return v.toLocaleDateString('en-US', {
								month: 'short',
								day: 'numeric'
							});
						}}
						indicator="line"
					/>
				{/snippet}
			</AreaChart>
		</Chart.Container>

        <!-- Manual Legend -->
		<div class="custom-legend">
			<div class="custom-legend-item">
				<span class="legend-dot" style="background:{chartConfig.generated.color}"></span>
				{chartConfig.generated.label}
			</div>
			<div class="custom-legend-item">
				<span class="legend-dot" style="background:{chartConfig.consumed.color}"></span>
				{chartConfig.consumed.label}
			</div>
		</div>
	</Card.Content>
</Card.Root>
