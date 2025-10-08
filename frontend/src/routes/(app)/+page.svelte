<script lang="ts">
	import CommunityCard from '$lib/components/CommunityCard.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import SquarePlus from '@lucide/svelte/icons/square-plus';
	import * as Select from '$lib/components/ui/select/index.js';
	import type { UserRole } from '$lib';

	const { data } = $props();

	let filterCommunityName = $state('');
	let selectedRoles = $state<UserRole[]>([]);

	let roles = [
		{ value: 'User', label: 'User' },
		{ value: 'Manager', label: 'Manager' }
	];

	const triggerRole = $derived(
		selectedRoles.length > 0
			? selectedRoles.map((value) => roles.find((role) => role.value === value)?.label).join(', ') || 'Select a role'
			: 'Select a role'
	);

	let filteredCommunities = $derived.by(() => {
		return data.communities
			.filter((c) => c.name.toLowerCase().includes(filterCommunityName.toLowerCase()))
			.filter((c) => selectedRoles.length === 0 || selectedRoles.includes(c.role));
		}
	);
</script>

<div class="flex flex-col gap-2 pb-4 md:flex-row md:items-center">
	<div class="order-2 w-full md:order-none">
		<SearchBar bind:filter={filterCommunityName} placeholder="Find community..." />
	</div>
	<div class="order-3 flex flex-row gap-2 md:order-none">
		<Select.Root type="multiple" name="selectRole" bind:value={selectedRoles}>
			<Select.Trigger class="w-full md:w-[180px]">
				{triggerRole}
			</Select.Trigger>
			<Select.Content>
				<Select.Group>
					{#each roles as role (role.value)}
						<Select.Item
						value={role.value}
						label={role.label}
						>
						{role.label}
						</Select.Item>
					{/each}
				</Select.Group>
			</Select.Content>
		</Select.Root>
	</div>
	<div class="order-1 md:order-none">
		<Button
			variant="outline"
			class={`w-full bg-green-600 text-sm text-white hover:bg-green-700 hover:text-white focus:outline-none md:text-base`}
			href="/community/new"
		>
			<SquarePlus />
			New Community
		</Button>
	</div>
</div>

<div class="grid grid-cols-1 gap-4">
	{#if filteredCommunities.length > 0}
		{#each filteredCommunities as community}
			<CommunityCard {community} />
		{/each}
	{:else}
		<div class="col-span-full flex flex-col items-center justify-center py-12">
			<p class="mb-2 text-lg font-medium text-gray-700 sm:text-2xl">
				You are not a member of any community
			</p>
		</div>
	{/if}
</div>
