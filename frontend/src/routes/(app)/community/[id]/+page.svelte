<script lang="ts">
	import ChartAreaInteractive from './ChartAreaInteractive.svelte';
	import DataTable from './DataTable.svelte';
	import EnergyBlocks from './EnergyBlocks.svelte';

	const { data } = $props();

	let paginatedEnergyRecords = $state(data.energyRecords);
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

	$effect(() => {
		loadEnergyRecords(pageIndex, pageSize);
	});
</script>

<svelte:head>
	<title>{data.communityId}</title>
</svelte:head>

<div class="container">
	<div class="flex flex-1 flex-col">
		<div class="@container/main flex flex-1 flex-col gap-2">
			<div class="flex flex-col gap-6">
				<EnergyBlocks energyRecords={paginatedEnergyRecords.records} />
				<ChartAreaInteractive />
				<DataTable data={paginatedEnergyRecords} bind:pageIndex bind:pageSize />
			</div>
		</div>
	</div>
</div>
