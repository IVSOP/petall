<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import UserPlus from '@tabler/icons-svelte/icons/user-plus';

	let {
		open = $bindable(),
		user_type,
		title
	}: {
		open: boolean;
		user_type: string;
		title: string;
	} = $props();
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>{title}</Dialog.Title>
			<Dialog.Description>
				Enter the email of the {user_type} you want to add. To revert this operation, simply remove the
				{user_type}
				afterwards.
			</Dialog.Description>
		</Dialog.Header>

		<form method="POST" action="?/addUser">
			<div class="grid gap-4 py-4">
				<div class="grid grid-cols-4 items-center gap-4">
					<Input type="hidden" name="user_type" value={user_type} />

					<Label for="user_email" class="text-right">Email</Label>
					<Input
						id="user_email"
						name="user_email"
						type="email"
						placeholder="{user_type}@example.com"
						class="col-span-3"
						required
					/>
				</div>

				<Dialog.Footer>
					<Button
						type="submit"
						class="flex w-full cursor-pointer items-center justify-center gap-2"
					>
						<UserPlus class="h-6 w-6" />
						Add {user_type}
					</Button>
				</Dialog.Footer>
			</div>
		</form>
	</Dialog.Content>
</Dialog.Root>
