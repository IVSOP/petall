<script lang="ts">
  import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
  import * as Chart from "$lib/components/ui/chart/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { scaleUtc } from "d3-scale";
  import { Area, AreaChart, ChartClipPath } from "layerchart";
  import { curveNatural } from "d3-shape";
  import ChartContainer from "$lib/components/ui/chart/chart-container.svelte";

  import { cubicInOut } from "svelte/easing";

  let { energyTransfers = [], participant_id = '' } = $props();

  let timeRange = $state("90d");

  const selectedLabel = $derived.by(() => {
    switch (timeRange) {
      case "90d":
        return "Last 3 months";
      case "30d":
        return "Last 30 days";
      case "7d":
        return "Last 7 days";
      default:
        return "Last 3 months";
    }
  });

  const filteredData = $derived(
    energyTransfers.map((item) => {
      return {
        date: new Date(item.start),
        generated: item.generated,
        consumed: item.consumed
      };
    })
  );

  const chartConfig = {
    generated: { label: "Generated", color: "var(--chart-1)" },
    consumed: { label: "Consumed", color: "var(--chart-2)" },
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
        <Select.Item value="90d" class="rounded-lg">Last 3 months</Select.Item>
        <Select.Item value="30d" class="rounded-lg">Last 30 days</Select.Item>
        <Select.Item value="7d" class="rounded-lg">Last 7 days</Select.Item>
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
            key: "consumed",
            label: "Consumed",
            color: chartConfig.consumed.color,
          },
          {
            key: "generated",
            label: "Generated",
            color: chartConfig.generated.color,
          },
        ]}
        seriesLayout="stack"
        props={{
          area: {
            curve: curveNatural,
            "fill-opacity": 0.4,
            line: { class: "stroke-1" },
            motion: "tween",
          },
          xAxis: {
            ticks: timeRange === "7d" ? 7 : undefined,
            format: (v) => {
              return v.toLocaleDateString("en-US", {
                month: "short",
                day: "numeric",
              });
            },
          },

          yAxis: { format: () => "" },
        }}
      >
        {#snippet marks({ series, getAreaProps })}
          <defs>
            <linearGradient id="fillGenerated" x1="0" y1="0" x2="0" y2="1">
              <stop
                offset="5%"
                stop-color="var(--color-generated)"
                stop-opacity={1.0}
              />
              <stop
                offset="95%"
                stop-color="var(--color-generated)"
                stop-opacity={0.1}
              />
            </linearGradient>
            <linearGradient id="fillConsumed" x1="0" y1="0" x2="0" y2="1">
              <stop offset="5%" stop-color="var(--color-consumed)" stop-opacity={0.8} />
              <stop
                offset="95%"
                stop-color="var(--color-consumed)"
                stop-opacity={0.1}
              />
            </linearGradient>
          </defs>
          <ChartClipPath
            initialWidth={0}
            motion={{
              width: { type: "tween", duration: 1000, easing: cubicInOut },
            }}
          >
            {#each series as s, i (s.key)}
              <Area
                {...getAreaProps(s, i)}
                fill={s.key === "generated"
                  ? "url(#fillGenerated)"
                  : "url(#fillConsumed)"}
              />
            {/each}
          </ChartClipPath>
        {/snippet}
        {#snippet tooltip()}
          <Chart.Tooltip
            labelFormatter={(v: Date) => {
              return v.toLocaleDateString("en-US", {
                month: "long",
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
        <div class="flex items-center gap-2 font-medium leading-none">
          Trending up by 5.2% this month <TrendingUpIcon class="size-4" />
        </div>
        <div class="text-muted-foreground flex items-center gap-2 leading-none">
          January - June 2024
        </div>
      </div>
    </div>
  </Card.Footer>
</Card.Root>
