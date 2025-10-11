<script lang="ts">
	import ChartAreaInteractive from './ChartAreaInteractive.svelte';
	import DataTable from './DataTable.svelte';
	import EnergyBlocks from './EnergyBlocks.svelte';
    import { format } from 'date-fns';
	import type { EnergyStats } from '$lib';
	import { onMount } from 'svelte';

	const { data } = $props();

	let paginatedEnergyRecords = $state(data.energyRecords);
    let stats_all = $state<EnergyStats | undefined>(undefined);
	let pageIndex = $state(1);
	let pageSize = $state(10);

	async function loadEnergyRecords(page: number, size: number) {
		const response = await fetch(`/api/community/${data.communityId}/energy`, {
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
        const response = await fetch(`/api/community/${data.communityId}/stats`, {
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

        if (response.ok) {
            const response_json = await response.json();
            console.log(response_json);
            return response_json;
        }
    }

	$effect(() => {
		loadEnergyRecords(pageIndex, pageSize);
	});

    $effect(() => {
		getStats(new Date(0), new Date(), "all").then((result) => {
			stats_all = result[0];
		});
	});
</script>

<svelte:head>
	<title>{data.communityId}</title>
</svelte:head>

<div class="container">
	<div class="flex flex-1 flex-col">
		<div class="@container/main flex flex-1 flex-col gap-2">
			<div class="flex flex-col gap-6">
				<!-- <EnergyBlocks energyRecords={paginatedEnergyRecords.records} /> -->
                 <EnergyBlocks statsAll={stats_all} />
				<ChartAreaInteractive />
				<DataTable data={paginatedEnergyRecords} bind:pageIndex bind:pageSize />
			</div>
		</div>
	</div>
</div>
