<script lang="ts" generics="TData, TValue">
	import * as Table from '$lib/components/ui/table/index.js';
	import * as Menubar from '$lib/components/ui/menubar/index.js';
	import * as Collapsible from '$lib/components/ui/collapsible/index.js';
	import { toast } from 'svelte-sonner';
	import { Button } from '$lib/components/ui/button/index.js';
	import { createSvelteTable, FlexRender } from '$lib/components/ui/data-table/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import Calendar from '$lib/components/ui/calendar/calendar.svelte';
	import { CalendarDate } from '@internationalized/date';
	import {
		type ColumnDef,
		type PaginationState,
		type RowSelectionState,
		getCoreRowModel,
		getPaginationRowModel,
		getFilteredRowModel
	} from '@tanstack/table-core';
	import { slide } from 'svelte/transition';

	type DataTableProps<TData, TValue> = {
		data: TData[];
		columns: ColumnDef<TData, TValue>[];
	};

	let { data, columns }: DataTableProps<TData, TValue> = $props();

	let showFilters = $state(false);

	let startDate: string = $state('');
	let endDate: string = $state('');
	let amount: number | '' = $state('');

	let value = $state<CalendarDate | undefined>(new CalendarDate(2025, 6, 12));
	let pagination = $state<PaginationState>({ pageIndex: 0, pageSize: 10 });
	let rowSelection = $state<RowSelectionState>({});

	const table = createSvelteTable({
		get data() {
			return data;
		},
		columns,
		getCoreRowModel: getCoreRowModel(),
		getPaginationRowModel: getPaginationRowModel(),
		getFilteredRowModel: getFilteredRowModel(),
		onPaginationChange: (updater) => {
			if (typeof updater === 'function') {
				pagination = updater(pagination);
			} else {
				pagination = updater;
			}
		},
		onRowSelectionChange: (updater) => {
			if (typeof updater === 'function') {
				rowSelection = updater(rowSelection);
			} else {
				rowSelection = updater;
			}
		},
		state: {
			get pagination() {
				return pagination;
			},
			get rowSelection() {
				return rowSelection;
			}
		}
	});
</script>

<div>
	<div class="flex items-center justify-between space-x-2">
		<Menubar.Root>
			<Menubar.Menu>
				<Menubar.Trigger>View</Menubar.Trigger>
				<Menubar.Content>
					<Menubar.Item>Today</Menubar.Item>
					<Menubar.Item>Last 7 days</Menubar.Item>
					<Menubar.Item>Last 30 days</Menubar.Item>
					<Menubar.Item>This year</Menubar.Item>
					<Menubar.Item>Older</Menubar.Item>
				</Menubar.Content>
			</Menubar.Menu>

			<Menubar.Menu>
				<Menubar.Trigger onclick={() => (showFilters = !showFilters)}>Filters</Menubar.Trigger>
			</Menubar.Menu>
			<Menubar.Menu>
				<Menubar.Trigger>Export</Menubar.Trigger>
				<Menubar.Content>
					<Menubar.Item>CSV</Menubar.Item>
					<Menubar.Item>JSON</Menubar.Item>
				</Menubar.Content>
			</Menubar.Menu>
		</Menubar.Root>
		<Button
			variant="outline"
			disabled={!table.getFilteredSelectedRowModel().rows.length}
			onclick={() =>
				toast.success('Event has been created', {
					description: 'Sunday, December 03, 2023 at 9:00 AM',
					action: {
						label: 'Undo',
						onClick: () => console.info('Undo')
					}
				})}
		>
			Validate
		</Button>
	</div>
	{#if showFilters}
		<div transition:slide={{ duration: 200 }}>
			<div class="grid grid-cols-3 gap-4 rounded-md border p-4 mt-4">
				<div>
					<Label for="start-date">Start Date</Label>
					<Input id="start-date" type="date" bind:value={startDate} />
				</div>
				<div>
					<Label for="end-date">End Date</Label>
					<Input id="end-date" type="date" bind:value={endDate} />
				</div>
				<div>
					<Label for="amount">Amount</Label>
					<Input id="amount" type="number" min="0" bind:value={amount} />
				</div>
			</div>
		</div>
	{/if}
	<div class="rounded-md border mt-4">
		<Table.Root>
			<Table.Header>
				{#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
					<Table.Row>
						{#each headerGroup.headers as header (header.id)}
							<Table.Head colspan={header.colSpan}>
								{#if !header.isPlaceholder}
									<FlexRender
										content={header.column.columnDef.header}
										context={header.getContext()}
									/>
								{/if}
							</Table.Head>
						{/each}
					</Table.Row>
				{/each}
			</Table.Header>
			<Table.Body>
				{#each table.getRowModel().rows as row (row.id)}
					<Table.Row data-state={row.getIsSelected() && 'selected'}>
						{#each row.getVisibleCells() as cell (cell.id)}
							<Table.Cell>
								<FlexRender content={cell.column.columnDef.cell} context={cell.getContext()} />
							</Table.Cell>
						{/each}
					</Table.Row>
				{:else}
					<Table.Row>
						<Table.Cell colspan={columns.length} class="h-24 text-center">No results.</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
	<div class="flex items-center justify-end space-x-2 py-4">
		<div class="flex-1 text-sm text-muted-foreground">
			{table.getFilteredSelectedRowModel().rows.length} of{' '}
			{table.getFilteredRowModel().rows.length} row(s) selected.
		</div>
		<Button
			variant="outline"
			size="sm"
			onclick={() => table.previousPage()}
			disabled={!table.getCanPreviousPage()}
		>
			Previous
		</Button>
		<Button
			variant="outline"
			size="sm"
			onclick={() => table.nextPage()}
			disabled={!table.getCanNextPage()}
		>
			Next
		</Button>
	</div>
</div>
