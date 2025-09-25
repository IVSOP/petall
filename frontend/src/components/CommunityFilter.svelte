<script lang="ts">
	import { Input } from '$lib/components/ui/input/index.js';
	import Search from '@lucide/svelte/icons/search';
	import CommunityModal from './CommunityModal.svelte';
	import * as Select from '$lib/components/ui/select/index.js';

	let { value = $bindable(), ...props } = $props();
</script>

<div class="flex flex-col gap-2 pb-4 md:flex-row md:items-center">
	<div class="relative order-2 md:order-none w-full">
		<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
			<Search class="h-4 w-4 text-muted-foreground" />
		</div>
		<Input
			bind:value
			{...props}
			placeholder="Find a community..."
			class="flex-1 pl-9 text-sm md:text-base"
		/>
	</div>
	<div class="order-3 flex gap-2 md:order-none">
		<Select.Root type="single">
			<Select.Trigger class="w-full text-sm md:text-base">Role</Select.Trigger>
			<Select.Content>
				<Select.Label class="text-sm">Role</Select.Label>
				<Select.Separator />
				<Select.Item value="User">User</Select.Item>
				<Select.Item value="Manager">Manager</Select.Item>
				<Select.Item value="UserManager">UserManager</Select.Item>
			</Select.Content>
		</Select.Root>
		<Select.Root type="single">
			<Select.Trigger class="w-full text-sm md:text-base">Sort</Select.Trigger>
			<Select.Content>
				<Select.Label class="text-sm">Sort</Select.Label>
				<Select.Separator />
				<Select.Item value="byDate">Date</Select.Item>
				<Select.Item value="byName">Name</Select.Item>
				<Select.Item value="byParticipants">Participants</Select.Item>
			</Select.Content>
		</Select.Root>
	</div>
	<CommunityModal />
</div>
