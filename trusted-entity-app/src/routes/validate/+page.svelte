<script lang="ts">
	import type { PageData } from './$types';
	import Zap from '@lucide/svelte/icons/zap';
	import CircleDashed from '@lucide/svelte/icons/circle-dashed';

	let { data }: { data: PageData } = $props();

	const profit: number = Number(data.energyRecordCost);
	const balance: number = data.energyRecordData.generated - data.energyRecordData.consumed;
	const start_date = new Date(data.energyRecordData.start).toLocaleString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
		hour: 'numeric',
		minute: '2-digit',
		hour12: true
	});
</script>

<svelte:head>
	<title>PeTall - {data.energyRecordData.id}</title>
</svelte:head>

<div>
	<div class="space-y-6 rounded-xl border-1 p-6 shadow-lg">
		<div class="flex items-start justify-between border-b border-slate-200 pb-4">
			<div>
				<h2 class="text-lg font-semibold text-slate-900">Energy Report</h2>
				<p class="mt-1 text-sm text-slate-500">{start_date}</p>
			</div>
			<div class="rounded-full bg-green-100 px-3 py-1 text-xs font-medium text-green-700">
				Validated
			</div>
		</div>
		
		<div class="grid grid-cols-2 gap-4">
			<div class="rounded-lg bg-green-50 p-4">
				<div class="mb-2 flex items-center gap-2">
					<Zap class="h-5 w-5 text-green-600" />
					<span class="text-sm font-medium text-green-900">Generated</span>
				</div>
				<p class="text-2xl font-bold text-green-700">{data.energyRecordData.generated}</p>
				<p class="mt-1 text-xs text-green-600">kWh</p>
			</div>

			<div class="rounded-lg bg-orange-50 p-4">
				<div class="mb-2 flex items-center gap-2">
					<CircleDashed class="h-5 w-5 text-orange-600" />
					<span class="text-sm font-medium text-orange-900">Consumed</span>
				</div>
				<p class="text-2xl font-bold text-orange-700">{data.energyRecordData.consumed}</p>
				<p class="mt-1 text-xs text-orange-600">kWh</p>
			</div>
		</div>

		<div class="rounded-lg bg-slate-50 p-4">
			<div class="flex items-center justify-between">
				<span class="text-sm font-medium text-slate-700">Energy Balance</span>
				<div class="flex items-center gap-2">
					<span
						class="text-xl font-bold"
						class:text-green-600={balance >= 0}
						class:text-red-600={balance < 0}
					>
						{balance > 0 ? '+' : ''}{balance.toFixed(4)} kWh
					</span>
					{#if balance >= 0}
						<span class="rounded bg-green-100 px-2 py-1 text-xs text-green-700">Surplus</span>
					{:else}
						<span class="rounded bg-red-100 px-2 py-1 text-xs text-red-700">Deficit</span>
					{/if}
				</div>
			</div>
		</div>

		<div class="space-y-3">
			<h3 class="text-md font-semibold text-slate-900">Pricing</h3>
			<div class="grid grid-cols-2 gap-3">
				<div class="flex items-center justify-between rounded-lg bg-blue-50 p-3">
					<span class="text-sm text-blue-900">Consumer</span>
					<span class="font-semibold text-blue-700">{data.energyRecordData.consumerPrice}/kWh</span>
				</div>
				<div class="flex items-center justify-between rounded-lg bg-purple-50 p-3">
					<span class="text-sm text-purple-900">Seller</span>
					<span class="font-semibold text-purple-700">{data.energyRecordData.sellerPrice}/kWh</span>
				</div>
			</div>
		</div>

		<div class="rounded-lg bg-slate-50 p-4">
			<div class="flex items-center justify-between">
				<span class="text-sm font-medium text-slate-700">Profit</span>
				<div class="flex items-center gap-2">
					<span
						class="text-xl font-bold"
						class:text-green-600={profit >= 0}
						class:text-red-600={profit < 0}
					>
						{profit > 0 ? '+' : ''}{profit} €
					</span>
					{#if profit >= 0}
						<span class="rounded bg-green-100 px-2 py-1 text-xs text-green-700">Surplus</span>
					{:else}
						<span class="rounded bg-red-100 px-2 py-1 text-xs text-red-700">Deficit</span>
					{/if}
				</div>
			</div>
		</div>

		<div class="space-y-2 border-t border-slate-200 pt-4">
			<div class="flex items-center justify-between text-xs">
				<span class="text-slate-500">User</span>
				<span class="font-mono text-slate-700">{data.energyRecordData.userId}</span>
			</div>
			<div class="flex items-center justify-between text-xs">
				<span class="text-slate-500">Community</span>
				<span class="font-mono text-slate-700">{data.energyRecordData.communityId}</span>
			</div>
			<div class="flex items-center justify-between text-xs">
				<span class="text-slate-500">Record</span>
				<span class="font-mono text-slate-700">{data.energyRecordData.id}</span>
			</div>
			<div class="flex items-center justify-between text-xs">
				<span class="text-slate-500">Proof</span>
				<span class="font-mono text-slate-700">{data.proof}</span>
			</div>
		</div>
	</div>
</div>
