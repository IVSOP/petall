<script lang="ts">
	import type { EnergyRecord } from '$lib';
	import EnergyChart from '$lib/components/EnergyChart.svelte';
	import type { PageProps } from './$types';
	import { columns } from './columns';
	import EnergyBlocks from './EnergyBlocks.svelte';
	import EnergyTable from './EnergyTable.svelte';

	const { data }: PageProps = $props();

	let energyTransfers: EnergyRecord[] = [
		{
			id: '10000',
			userId: '10000',
			communityId: '10000',
			generated: 1.254,
			consumed: 12.5,
			consumerPrice: 0.125,
			sellerPrice: 0.125,
			start: '2023-01-01T00:00:00Z'
		},
		{
			id: '10001',
			userId: '10001',
			communityId: '10001',
			generated: 2.508,
			consumed: 25,
			consumerPrice: 0.25,
			sellerPrice: 0.25,
			start: '2023-01-01T00:00:00Z'
		},
		{
			id: '10002',
			userId: '10002',
			communityId: '10002',
			generated: 3.762,
			consumed: 37.5,
			consumerPrice: 0.375,
			sellerPrice: 0.375,
			start: '2023-01-01T00:00:00Z'
		}
	];
</script>

<div class="container">
	<div class="grid grid-cols-1 gap-4">

        <div class="flex flex-1 flex-col">
            <div class="@container/main flex flex-1 flex-col gap-2">
                <div class="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
                    <EnergyBlocks energyTransfers={energyTransfers}/>
                </div>
            </div>
        </div>

        <!-- <EnergyChart energyTransfers={data.energyTransfers} participant_id={data.participantId} /> -->
		<EnergyTable data={energyTransfers} {columns} />
	</div>
</div>
