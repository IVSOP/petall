<script lang="ts">
	import EnergyChart from '$lib/components/EnergyChart.svelte';
	import EnergyTable from '$lib/components/EnergyTable.svelte';
	import type { PageProps } from './$types';
	import { columns } from './columns';

	const { data }: PageProps = $props();
</script>

<div class="container">
	<div class="grid grid-cols-1 gap-4">
		<EnergyChart energyTransfers={data.energyTransfers} participant_id={data.participantId} />
		<EnergyTable data={data.energyTransfers} {columns} />
	</div>
</div>
