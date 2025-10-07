<script lang="ts">
	import SectionCards from "$lib/components/SectionCards.svelte";
	import ChartAreaInteractive from "$lib/components/ChartAreaInteractive.svelte";
	import DataTable from "$lib/components/DataTable.svelte";

	const { data } = $props();

</script>


<div class="container">
	<div class="flex flex-1 flex-col">
		<div class="@container/main flex flex-1 flex-col gap-2">
			<div class="flex flex-col gap-6">
				<SectionCards />
				<ChartAreaInteractive />
				<DataTable data={data.energyRecords}/>
			</div>
		</div>
    </div>
</div>
