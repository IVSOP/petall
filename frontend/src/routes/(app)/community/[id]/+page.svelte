<script lang="ts">
	import SectionCards from "$lib/components/SectionCards.svelte";
	import ChartAreaInteractive from "$lib/components/ChartAreaInteractive.svelte";
	import DataTable from "$lib/components/DataTable.svelte";

	const { data } = $props();

	let energyRecords = $state(data.energyRecords);
	let pageIndex = $state(1);
	let pageSize = $state(10);
	let pageLimit = $state(5);

	async function loadEnergyRecords(page: number, size: number) {
		console.log("aqui");
		const response = await fetch(`/api/community/${data.communityId}/energy`, {
			method: 'POST',
			headers: {
				Authorization: data.sessionId,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				"page": page,
				"size": size,
				"orderDir": "desc"
			})
		});

		console.log(`index: ${page}`);
		console.log(`size: ${size}`);
		console.log(response.ok)

		if (response.ok) {
			energyRecords = await response.json();
		}
	}

	$effect(() => {
		loadEnergyRecords(pageIndex, pageSize);
	});
</script>


<div class="container">
	<div class="flex flex-1 flex-col">
		<div class="@container/main flex flex-1 flex-col gap-2">
			<div class="flex flex-col gap-6">
				<SectionCards />
				<ChartAreaInteractive />
				<DataTable data={energyRecords} bind:pageIndex={pageIndex} bind:pageLimit={pageLimit} bind:pageSize={pageSize}/>
			</div>
		</div>
    </div>
</div>
