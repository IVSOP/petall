<script lang="ts">
	import ChartAreaInteractive from './ChartAreaInteractive.svelte';
	import DataTable from './DataTable.svelte';
	import EnergyBlocks from './EnergyBlocks.svelte';
	import { format } from 'date-fns';
	import type { EnergyStats } from '$lib';

	const { data } = $props();

	let paginatedEnergyRecords = $state(data.energyRecords);
	let stats_all = $state<EnergyStats | undefined>(undefined);
	let stats_last_60 = $state<EnergyStats[] | undefined>(undefined);
	let pageIndex = $state(1);
	let pageSize = $state(10);

	async function loadEnergyRecords(page: number, size: number) {
		const response = await fetch(`/api/community/${data.community.id}/energy`, {
			method: 'POST',
			headers: {
				Authorization: data.sessionId,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				page: page,
				size: size,
				orderDir: 'desc'
			})
		});

		console.log(`index: ${page}`);
		console.log(`size: ${size}`);
		console.log(response.ok);

		if (response.ok) {
			paginatedEnergyRecords = await response.json();
		}
	}

	async function getStats(start: Date, end: Date, granularity: String) {
		const response = await fetch(`/api/community/${data.community.id}/stats`, {
			method: 'POST',
			headers: {
				Authorization: data.sessionId,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				start: format(start, "yyyy-MM-dd'T'HH:mm:ss"),
				end: format(end, "yyyy-MM-dd'T'HH:mm:ss"),
				granularity
			})
		});

		if (!response.ok) {
			throw new Error(`Failed to fetch stats: ${response.statusText}`);
		}

		const response_json = await response.json();

		// string to date
		return response_json.map((s: any) => ({
			...s,
			periodStart: new Date(s.periodStart)
		}));
	}

	$effect(() => {
		loadEnergyRecords(pageIndex, pageSize);
	});

	$effect(() => {
		const today = new Date();
		getStats(new Date(0), today, 'all').then((result) => {
			stats_all = result[0];
		});
		let date_60_days_ago = new Date();
		date_60_days_ago.setDate(today.getDate() - 60);
		getStats(date_60_days_ago, today, 'daily').then((result) => {
			stats_last_60 = result;
		});
	});
</script>

<svelte:head>
	<title>{data.community.name}</title>
</svelte:head>

<div class="container">
	<div class="flex flex-1 flex-col">
		<div class="@container/main flex flex-1 flex-col gap-2">
			<div class="flex flex-col gap-6">
				<EnergyBlocks statsLast60={stats_last_60} />
				<ChartAreaInteractive statsLast60={stats_last_60} />
				<DataTable data={paginatedEnergyRecords} bind:pageIndex bind:pageSize />
			</div>
		</div>
	</div>
</div>
