<script lang="ts">
	import Profile from './Profile.svelte';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

	// TODO: participant object should have image
	let { participant, image } = $props();
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger class="focus:outline-none">
		<Avatar.Root class="cursor-pointer transition hover:scale-105 hover:ring-2 hover:ring-gray-300">
			<Avatar.Image src={image} />
			<Avatar.Fallback>A</Avatar.Fallback>
		</Avatar.Root>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Label>{participant.name}</DropdownMenu.Label>
			<DropdownMenu.Separator />
			<Profile />
			<DropdownMenu.Item>
				<a href="/logout" class="w-full">Logout</a>
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
