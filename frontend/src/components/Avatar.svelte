<script lang="ts">
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

	let { image } = $props();
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		<Avatar.Root class="cursor-pointer hover:ring-2 hover:ring-gray-300 hover:scale-105 transition">
			<Avatar.Image src={image} />
			<Avatar.Fallback>A</Avatar.Fallback>
		</Avatar.Root>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Label>NomeQualquer</DropdownMenu.Label>
			<DropdownMenu.Separator />
            <DropdownMenu.Item>
				<a href="/profile" class="w-full">Profile</a>
			</DropdownMenu.Item>
			<DropdownMenu.Item>
				<a href="/change-password" class="w-full">Change Password</a>
            </DropdownMenu.Item>
			<DropdownMenu.Item>
				<a href="/logout" class="w-full">Logout</a>
            </DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
