import type { EnergyTransfer } from '$lib';
import type { ColumnDef } from '@tanstack/table-core';

export const columns: ColumnDef<EnergyTransfer>[] = [
	{
		accessorKey: 'participant_from',
		header: 'From'
	},
	{
		accessorKey: 'participant_to',
		header: 'To'
	},
	{
		accessorKey: 'start',
		header: 'Start'
	},
	{
		accessorKey: 'end',
		header: 'End'
	},
	{
		accessorKey: 'energy_wh',
		header: 'Energy WH'
	}
];
