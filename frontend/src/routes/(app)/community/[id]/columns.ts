import type { EnergyRecord } from '$lib';
import type { ColumnDef } from '@tanstack/table-core';
import { createRawSnippet } from 'svelte';
import { renderComponent } from '$lib/components/ui/data-table/index.js';
import { renderSnippet } from '$lib/components/ui/data-table/index.js';
import { Checkbox } from '$lib/components/ui/checkbox/index.js';
import { format } from 'date-fns';

export const columns: ColumnDef<EnergyRecord>[] = [
	{
		accessorKey: 'start',
		header: 'Start',
		cell: ({ row }) => {
			return format(row.getValue('start'), 'd/M/yyyy HH:mm');
		}
	},
	{
		accessorKey: 'generated',
		header: () => {
			const energyHeaderSnippet = createRawSnippet(() => ({
				render: () => `<div class="text-right">Energy Generated (Wh)</div>`
			}));
			return renderSnippet(energyHeaderSnippet, '');
		},
		cell: ({ row }) => {
			const formatter = new Intl.NumberFormat('en-US', {
				minimumFractionDigits: 2,
				maximumFractionDigits: 2
			});

			const energyCellSnippet = createRawSnippet<[string]>((getEnergyWh) => {
				const generated_wh = getEnergyWh();

				return {
					render: () => {
						return `
						<div class="box-content flex justify-end">
							<div class="inline-block text-right font-medium py-1.5 px-3 rounded-md text-sm">${generated_wh}</div>
						</div>`;
					}
				};
			});

			return renderSnippet(
				energyCellSnippet,
				formatter.format(parseFloat(row.getValue('generated')))
			);
		}
	},
	{
		accessorKey: 'consumed',
		header: () => {
			const energyHeaderSnippet = createRawSnippet(() => ({
				render: () => `<div class="text-right">Energy Consumed (Wh)</div>`
			}));
			return renderSnippet(energyHeaderSnippet, '');
		},
		cell: ({ row }) => {
			const formatter = new Intl.NumberFormat('en-US', {
				minimumFractionDigits: 2,
				maximumFractionDigits: 2
			});

			const energyCellSnippet = createRawSnippet<[string]>((getEnergyWh) => {
				const consumed_wh = getEnergyWh();

				return {
					render: () => {
						return `
						<div class="box-content flex justify-end">
							<div class="inline-block text-right font-medium py-1.5 px-3 rounded-md text-sm">${consumed_wh}</div>
						</div>`;
					}
				};
			});

			return renderSnippet(
				energyCellSnippet,
				formatter.format(parseFloat(row.getValue('consumed')))
			);
		}
	},
	{
		id: 'delta',
		header: () => {
			const energyHeaderSnippet = createRawSnippet(() => ({
				render: () => `<div class="text-right">Energy Δ (Wh)</div>`
			}));
			return renderSnippet(energyHeaderSnippet, '');
		},
		cell: ({ row }) => {
			const formatter = new Intl.NumberFormat('en-US', {
				minimumFractionDigits: 2,
				maximumFractionDigits: 2
			});

			const energyCellSnippet = createRawSnippet<[number]>((getDelta) => {
				const delta = getDelta();
				const colorClass = delta < 0 ? 'bg-red-100' : 'bg-green-100';
				const textColorClass = delta < 0 ? 'text-red-500' : 'text-green-500';

				return {
					render: () => {
						return `
						<div class="box-content flex justify-end">
							<div class="inline-block text-right font-medium ${colorClass} ${textColorClass} py-1.5 px-3 rounded-md text-sm">${delta}</div>
						</div>`;
					}
				};
			});

			return renderSnippet(energyCellSnippet, row.original.generated - row.original.consumed);
		}
	}
];
