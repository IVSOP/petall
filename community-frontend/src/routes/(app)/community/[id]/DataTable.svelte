<script lang="ts">
	import type { PaginatedEnergyRecords } from '$lib/api/community.js';
	import type { ColumnDef, Row, VisibilityState } from '@tanstack/table-core';
	import { getCoreRowModel } from '@tanstack/table-core';
	import { createSvelteTable } from '$lib/components/ui/data-table/data-table.svelte.js';
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import * as Table from '$lib/components/ui/table/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { FlexRender, renderSnippet } from '$lib/components/ui/data-table/index.js';
	import CircleCkeck from '@lucide/svelte/icons/circle-check';
	import LayoutColumnsIcon from '@tabler/icons-svelte/icons/layout-columns';
	import ChevronDownIcon from '@tabler/icons-svelte/icons/chevron-down';
	import ChevronsLeftIcon from '@tabler/icons-svelte/icons/chevrons-left';
	import ChevronLeftIcon from '@tabler/icons-svelte/icons/chevron-left';
	import ChevronRightIcon from '@tabler/icons-svelte/icons/chevron-right';
	import ChevronsRightIcon from '@tabler/icons-svelte/icons/chevrons-right';
	import type { EnergyRecord } from '$lib';
	import { handleValidate } from '$lib/api/validate';

	const columns: ColumnDef<EnergyRecord>[] = [
		{
			accessorKey: 'start',
			header: 'Time',
			cell: ({ row }) => renderSnippet(DataTableTime, { row }),
			enableHiding: true
		},
		{
			accessorKey: 'consumed',
			header: 'Consumed Energy (kWh)',
			enableHiding: true
		},
		{
			accessorKey: 'generated',
			header: 'Generated Energy (kWh)',
			enableHiding: true
		},
		{
			accessorKey: 'consumerPrice',
			header: 'Consumer Price (€/kWh)',
			enableHiding: true
		},
		{
			accessorKey: 'sellerPrice',
			header: 'Seller Price (€/kWh)',
			enableHiding: true
		},
		{
			accessorFn: (row) => row.generated - row.consumed,
			id: 'energyBalance',
			header: 'Energy Balance (kWh)',
			cell: ({ row }) => renderSnippet(DataTableEnergyBalance, { row }),
			enableHiding: true
		},
		{
			accessorFn: (row) => row.generated * row.sellerPrice - row.consumed * row.consumerPrice,
			id: 'profit',
			header: 'Profit (€)',
			cell: ({ row }) => renderSnippet(DataTableProfit, { row }),
			enableHiding: true
		},
		{
			id: 'actions',
			cell: ({ row }) => renderSnippet(DataTableActions, { row })
		}
	];

	let {
		data,
		pageIndex = $bindable(),
		pageSize = $bindable(),
		session_id,
		...props
	}: {
		data: PaginatedEnergyRecords;
		pageIndex: number;
		pageSize: number;
		session_id: string;
	} = $props();

	const totalPages = $derived(Math.ceil(data.totalCount / pageSize));

	$effect(() => {
		if (pageIndex > totalPages) {
			pageIndex = totalPages;
		}
	});

	let columnVisibility = $state<VisibilityState>({});

	const table = createSvelteTable({
		get data() {
			return data.records;
		},
		columns,
		state: {
			get columnVisibility() {
				return columnVisibility;
			}
		},
		getRowId: (row) => row.id,
		getCoreRowModel: getCoreRowModel(),
		onColumnVisibilityChange: (updater) => {
			columnVisibility = typeof updater === 'function' ? updater(columnVisibility) : updater;
		}
	});

	let views = [
		{
			id: 'energy_records_table',
			label: 'Detailed Information',
			badge: 0
		}
	];
</script>

<Tabs.Root value="energy_records_table" class="w-full flex-col justify-start gap-4">
	<div class="flex items-center justify-between">
		<Tabs.List
			class="hidden **:data-[slot=badge]:size-5 **:data-[slot=badge]:rounded-full **:data-[slot=badge]:bg-muted-foreground/30 **:data-[slot=badge]:px-1 @md/main:flex"
		>
			{#each views as view (view.id)}
				<Tabs.Trigger value={view.id}>
					{view.label}
					{#if view.badge > 0}
						<Badge variant="secondary">{view.badge}</Badge>
					{/if}
				</Tabs.Trigger>
			{/each}
		</Tabs.List>
		<div class="flex items-center gap-2">
			<DropdownMenu.Root>
				<DropdownMenu.Trigger>
					{#snippet child({ props })}
						<Button variant="outline" size="sm" class="cursor-pointer" {...props}>
							<LayoutColumnsIcon />
							<span class="hidden lg:inline">Customize Columns</span>
							<span class="lg:hidden">Columns</span>
							<ChevronDownIcon />
						</Button>
					{/snippet}
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end" class="w-56">
					{#each table
						.getAllColumns()
						.filter((col) => typeof col.accessorFn !== 'undefined' && col.getCanHide()) as column (column.id)}
						<DropdownMenu.CheckboxItem
							class="cursor-pointer capitalize"
							checked={column.getIsVisible()}
							onCheckedChange={(value) => column.toggleVisibility(!!value)}
						>
							{column.id}
						</DropdownMenu.CheckboxItem>
					{/each}
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</div>
	</div>
	<Tabs.Content value="energy_records_table" class="relative flex flex-col gap-4 overflow-auto">
		<div class="overflow-hidden rounded-lg border">
			<Table.Root>
				<Table.Header class="sticky top-0 z-10 bg-muted">
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
				<Table.Body class="**:data-[slot=table-cell]:first:w-8">
					{#if table.getRowModel().rows?.length}
						{#each table.getRowModel().rows as row (row.id)}
							{@render TableRow({ row })}
						{/each}
					{:else}
						<Table.Row>
							<Table.Cell colspan={columns.length} class="h-24 text-center">No results.</Table.Cell>
						</Table.Row>
					{/if}
				</Table.Body>
			</Table.Root>
		</div>
		<div class="flex items-center justify-end">
			<div class="flex w-full items-center gap-8 lg:w-fit">
				<div class="hidden items-center gap-2 lg:flex">
					<Label for="rows-per-page" class="text-sm font-medium">Rows per page</Label>
					<Select.Root
						type="single"
						bind:value={() => `${pageSize}`, (v) => (pageSize = Number(v))}
					>
						<Select.Trigger size="sm" class="w-20 cursor-pointer" id="rows-per-page">
							{pageSize}
						</Select.Trigger>
						<Select.Content side="top">
							{#each [10, 20, 30, 40, 50] as pageSize (pageSize)}
								<Select.Item value={pageSize.toString()} class="cursor-pointer">
									{pageSize}
								</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>
				<div class="flex w-fit items-center justify-center text-sm font-medium">
					Page {pageIndex} of
					{totalPages}
				</div>
				<div class="ml-auto flex items-center gap-2 lg:ml-0">
					<Button
						variant="outline"
						class="hidden h-8 w-8 cursor-pointer p-0 lg:flex"
						onclick={() => (pageIndex = 1)}
						disabled={pageIndex === 1}
					>
						<span class="sr-only">Go to first page</span>
						<ChevronsLeftIcon />
					</Button>
					<Button
						variant="outline"
						class="size-8 cursor-pointer"
						size="icon"
						onclick={() => pageIndex--}
						disabled={pageIndex === 1}
					>
						<span class="sr-only">Go to previous page</span>
						<ChevronLeftIcon />
					</Button>
					<Button
						variant="outline"
						class="size-8 cursor-pointer"
						size="icon"
						onclick={() => pageIndex++}
						disabled={pageIndex >= totalPages}
					>
						<span class="sr-only">Go to next page</span>
						<ChevronRightIcon />
					</Button>
					<Button
						variant="outline"
						class="hidden size-8 cursor-pointer lg:flex"
						size="icon"
						onclick={() => (pageIndex = totalPages)}
						disabled={pageIndex >= totalPages}
					>
						<span class="sr-only">Go to last page</span>
						<ChevronsRightIcon />
					</Button>
				</div>
			</div>
		</div>
	</Tabs.Content>
</Tabs.Root>

{#snippet TableRow({ row }: { row: Row<EnergyRecord> })}
	<Table.Row>
		{#each row.getVisibleCells() as cell (cell.id)}
			<Table.Cell>
				<FlexRender content={cell.column.columnDef.cell} context={cell.getContext()} />
			</Table.Cell>
		{/each}
	</Table.Row>
{/snippet}

{#snippet DataTableTime({ row }: { row: Row<EnergyRecord> })}
	<div class="text-muted-foreground">
		{new Date(row.original.start).toLocaleString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: 'numeric',
			minute: '2-digit',
			hour12: true
		})}
	</div>
{/snippet}

{#snippet DataTableEnergyBalance({ row }: { row: Row<EnergyRecord> })}
	<div
		class="w-32 rounded px-2 py-1 text-center {row.original.generated - row.original.consumed < 0
			? 'bg-red-200 text-red-800'
			: 'bg-green-200 text-green-800 '}"
	>
		{(row.original.generated - row.original.consumed).toFixed(4)}
	</div>
{/snippet}

{#snippet DataTableProfit({ row }: { row: Row<EnergyRecord> })}
	<div
		class="w-32 rounded px-2 py-1 text-center {row.original.generated * row.original.sellerPrice -
			row.original.consumed * row.original.consumerPrice <
		0
			? 'bg-red-200 text-red-800'
			: 'bg-green-200 text-green-800'}"
	>
		{(
			row.original.generated * row.original.sellerPrice -
			row.original.consumed * row.original.consumerPrice
		).toFixed(4)}
	</div>
{/snippet}

{#snippet DataTableActions({ row }: { row: Row<EnergyRecord> })}
	<Button
		variant="outline"
		size="sm"
		class="cursor-pointer"
		onclick={() => handleValidate(row.original, session_id)}
	>
		<CircleCkeck />
	</Button>
{/snippet}
