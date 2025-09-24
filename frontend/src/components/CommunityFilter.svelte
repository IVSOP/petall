<script lang="ts">
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import CommunityModal from './CommunityModal.svelte';

	let { value = $bindable(), ...props } = $props();
</script>

<div class="flex flex-col gap-2 pb-4 md:flex-row md:items-center">
	<Input
		bind:value
		{...props}
		placeholder="Find a community..."
		class="md:text-md order-2 flex-1 text-base md:order-none"
	/>
	<div class="order-3 flex gap-2 md:order-none">
		<Select.Root type="single">
			<Select.Trigger class="w-full">Role</Select.Trigger>
			<Select.Content>
				<Select.Label>Role</Select.Label>
				<Select.Separator />
				<Select.Item value="User">User</Select.Item>
				<Select.Item value="Manager">Manager</Select.Item>
				<Select.Item value="UserManager">UserManager</Select.Item>
			</Select.Content>
		</Select.Root>
		<Select.Root type="single">
			<Select.Trigger class="w-full">Sort</Select.Trigger>
			<Select.Content>
				<Select.Label>Sort</Select.Label>
				<Select.Separator />
				<Select.Item value="byDate">Date</Select.Item>
				<Select.Item value="byName">Name</Select.Item>
				<Select.Item value="byParticipants">Participants</Select.Item>
			</Select.Content>
		</Select.Root>
	</div>
	<CommunityModal />
</div>
