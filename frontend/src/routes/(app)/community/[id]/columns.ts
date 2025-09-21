import type { Energy } from '$lib';
import type { ColumnDef } from '@tanstack/table-core';
import { createRawSnippet } from 'svelte';
import { renderComponent } from '$lib/components/ui/data-table/index.js';
import { renderSnippet } from '$lib/components/ui/data-table/index.js';
import { Checkbox } from '$lib/components/ui/checkbox/index.js';

export const columns: ColumnDef<Energy>[] = [
	{
		id: 'select',
		header: ({ table }) =>
			renderComponent(Checkbox, {
				checked: table.getIsAllPageRowsSelected(),
				indeterminate: table.getIsSomePageRowsSelected() && !table.getIsAllPageRowsSelected(),
				onCheckedChange: (value) => table.toggleAllPageRowsSelected(!!value),
				'aria-label': 'Select all'
			}),
		cell: ({ row }) =>
			renderComponent(Checkbox, {
				checked: row.getIsSelected(),
				onCheckedChange: (value) => row.toggleSelected(!!value),
				'aria-label': 'Select row'
			}),
		enableSorting: false,
		enableHiding: false
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
		accessorKey: 'delta',
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

			const energyCellSnippet = createRawSnippet<[string]>((getEnergyWh) => {
				const delta_wh = getEnergyWh();
				const colorClass = parseFloat(delta_wh) < 0 ? 'bg-red-100' : 'bg-green-100';
				const textColorClass = parseFloat(delta_wh) < 0 ? 'text-red-500' : 'text-green-500';

				return {
					render: () => {
						return `
						<div class="box-content flex justify-end">
							<div class="inline-block text-right font-medium ${colorClass} ${textColorClass} py-1.5 px-3 rounded-md text-sm">${delta_wh}</div>
						</div>`;
					}
				};
			});

			return renderSnippet(
				energyCellSnippet,
				formatter.format(parseFloat(row.getValue('delta')))
			);
		}
	},

];
