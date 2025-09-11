<script lang="ts">
	import { scaleBand } from 'd3-scale';
	import { BarChart } from 'layerchart';
	import TrendingUpIcon from '@lucide/svelte/icons/trending-up';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as Card from '$lib/components/ui/card/index.js';

	let { energyTransfers = [], participant_id = '' } = $props();

	const chartConfig = {
		energy_wh: { label: 'Energy WH' }
	} satisfies Chart.ChartConfig;
</script>

<Card.Root>
	<Card.Header class="flex items-center gap-2 space-y-0 border-b sm:flex-row">
		<div class="grid flex-1 gap-1 text-center sm:text-left">
			<Card.Title>Energy Transfers</Card.Title>
			<Card.Description>January - June 2024</Card.Description>
		</div>
	</Card.Header>
	<Card.Content>
		<Chart.Container config={chartConfig}>
			<BarChart
				labels={{
					offset: 5,
					value: (d) => d.start,
					fill: (d) => {
						if (d.energy_wh > 0) {
							return 'var(--chart-1)';
						} else if (d.energy_wh < 0) {
							return 'var(--chart-2)';
						}
					}
				}}
				data={energyTransfers}
				xScale={scaleBand().padding(0.25)}
				x="start"
				y="energy_wh"
				yNice={4}
				yBaseline={0}
				cRange={['var(--chart-1)', 'var(--chart-2)']}
				c={(d) => (d.energy_wh > 0 ? 'var(--chart-1)' : 'var(--chart-2)')}
				axis={false}
				props={{
					bars: { stroke: 'none', radius: 0 },
					highlight: { area: { fill: 'none' } },
					xAxis: { format: (d) => d.slice(0, 3) }
				}}
			>
				{#snippet tooltip()}
					<Chart.Tooltip hideLabel hideIndicator nameKey="visitors" />
				{/snippet}
			</BarChart>
		</Chart.Container>
	</Card.Content>

	<Card.Footer>
		<div class="flex w-full items-start gap-2 text-sm">
			<div class="grid gap-2">
				<div class="flex items-center gap-2 leading-none font-medium">
					Trending up by 5.2% this month <TrendingUpIcon class="size-4" />
				</div>
				<div class="flex items-center gap-2 leading-none text-muted-foreground">
					Showing total visitors for the last 6 months
				</div>
			</div>
		</div>
	</Card.Footer>
</Card.Root>
