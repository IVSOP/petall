<script lang="ts">
	import ProfileDialog from './ProfileDialog.svelte';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

	let { name }: { name: string } = $props();
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger class="focus:outline-none">
		<Avatar.Root class="cursor-pointer transition hover:scale-102 hover:ring-2 hover:ring-gray-300">
			<Avatar.Fallback>{name.charAt(0).toUpperCase()}</Avatar.Fallback>
		</Avatar.Root>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Label>{name}</DropdownMenu.Label>
			<DropdownMenu.Separator />
			<ProfileDialog />
			<DropdownMenu.Item>
				<a href="/logout" class="w-full">Logout</a>
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
