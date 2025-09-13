<script lang="ts">
	import type { PageProps } from './$types';
	import EnergyTable from '../../../../components/EnergyTable.svelte';
	import EnergyChart from '../../../../components/EnergyChart.svelte';
	import { columns } from './columns';
    import CustomChart from "../../../../components/Chart.svelte";

	const { data }: PageProps = $props();
</script>

<div class="container">
	<div class="grid grid-cols-1 gap-4">
		<EnergyChart energyTransfers={data.energyTransfers} participant_id={data.participantId} />
		<CustomChart/>
		<EnergyTable data={data.energyTransfers} {columns} />
	</div>
</div>
