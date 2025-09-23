<script lang="ts">
	import SquarePlus from '@lucide/svelte/icons/square-plus';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Select from '$lib/components/ui/select/index.js';

	export let value: string;
</script>

<div class="flex flex-col gap-2 pb-4 md:flex-row md:items-center">
	<Input
		bind:value
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
	<Button
		class="order-1 cursor-pointer border-none bg-green-600 text-white hover:bg-green-700 md:order-none"
	>
		<SquarePlus />
		New Community
	</Button>
</div>

<!-- <div class="flex w-full items-center pb-4">
	<Input
		bind:value
		placeholder="Find a community..."
		class="w-full text-base sm:text-lg md:text-lg lg:text-xl"
	/>
</div> -->
