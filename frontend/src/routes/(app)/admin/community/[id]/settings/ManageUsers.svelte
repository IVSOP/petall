<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import Users from '@lucide/svelte/icons/users';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import Trash_2 from '@lucide/svelte/icons/trash-2';
	import UserPlus from '@tabler/icons-svelte/icons/user-plus';
	import type { User } from '$lib';

	let {
		open = $bindable(),
		users,
		admins,
		title,
		user_type
	}: {
		open: boolean;
		users: User[];
		admins: User[];
		title: string;
		user_type: string;
	} = $props();
</script>

<Card.Root class="w-full gap-0 pb-0 lg:w-1/2">
	<Card.Header class="flex items-center justify-between border-b px-3">
		<div class="flex items-center gap-2">
			<Users />
			<h2 class="text-lg font-semibold">
				{title}
			</h2>
			<span class="text-sm text-muted-foreground">
				{users.length} users
			</span>
		</div>
		<Button class="cursor-pointer" onclick={() => (open = true)}>
			<UserPlus />
			Add {user_type}
		</Button>
	</Card.Header>

	<Card.Content class="max-h-[325px] overflow-hidden p-0">
		<ScrollArea class="h-[325px] w-full rounded-lg">
			{#each users as user}
				<div
					class="flex items-center justify-between gap-3 border-b p-3 transition-colors hover:bg-accent/50"
				>
					<div class="flex items-center gap-3">
						<Avatar.Root class="h-10 w-10">
							<Avatar.Fallback class="bg-primary/10 text-sm font-medium">
								{user.name.slice(0, 2).toUpperCase()}
							</Avatar.Fallback>
						</Avatar.Root>
						<span class="font-medium">{user.email}</span>
					</div>

					{#if admins.some((u) => u.id !== user.id) || user_type === 'user'}
						<form method="POST" action="?/removeUser">
							<Input type="hidden" name="user_type" value={user_type} />
							<Input type="hidden" name="user_email" value={user.email} />
							<Button
								type="submit"
								variant="destructive"
								class="cursor-pointer text-sm font-medium"
							>
								<Trash_2 class="mr-1" />
								Remove
							</Button>
						</form>
					{:else}
						<Button
							variant="destructive"
							class="cursor-not-allowed text-sm font-medium opacity-50 hover:cursor-not-allowed"
						>
							<Trash_2 class="mr-1" />
							Remove
						</Button>
					{/if}
				</div>
			{/each}
		</ScrollArea>
	</Card.Content>
</Card.Root>
