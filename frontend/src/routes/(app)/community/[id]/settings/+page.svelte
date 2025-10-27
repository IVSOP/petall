<script lang="ts">
	import type { PageProps } from './$types';
	import { v4 as uuidv4 } from 'uuid';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import Users from '@lucide/svelte/icons/users';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import Trash_2 from '@lucide/svelte/icons/trash-2';
	import UserPlus from '@tabler/icons-svelte/icons/user-plus';
	import AddMemberDialog from './AddMemberDialog.svelte';
	import type { Community } from '$lib';
	import Pencil from '@lucide/svelte/icons/pencil';
	import Save from '@lucide/svelte/icons/save';

	const { data }: PageProps = $props();

	let editing: boolean = $state(false);
	let community: Community = $state(data.community);

	let addUserDialogOpen: boolean = $state(false);
	let addManagerDialogOpen: boolean = $state(false);

	$inspect(community);

	const communityData = {
		name: 'Creative Community',
		rule: 'Be respectful and creative',
		image:
			'https://media.istockphoto.com/id/814423752/photo/eye-of-model-with-colorful-art-make-up-close-up.jpg?s=612x612&w=0&k=20&c=l15OdMWjgCKycMMShP8UK94ELVlEGvt7GmB_esHWPYE=',
		users: [
			{ id: uuidv4(), name: 'Chico Silva', email: 'chico.silva@example.com' },
			{ id: uuidv4(), name: 'Pedro Santos', email: 'pedro.santos@example.com' },
			{ id: uuidv4(), name: 'Diego Rocha', email: 'diego.rocha@example.com' },
			{ id: uuidv4(), name: 'Isabel Vieira', email: 'isabel.vieira@example.com' },
			{ id: uuidv4(), name: 'Carla Nunes', email: 'carla.nunes@example.com' },
			{ id: uuidv4(), name: 'Lucas Pereira', email: 'lucas.pereira@example.com' },
			{ id: uuidv4(), name: 'Fernanda Costa', email: 'fernanda.costa@example.com' },
			{ id: uuidv4(), name: 'Marcos Lima', email: 'marcos.lima@example.com' },
			{ id: uuidv4(), name: 'Renata Alves', email: 'renata.alves@example.com' },
			{ id: uuidv4(), name: 'Thiago Souza', email: 'thiago.souza@example.com' },
			{ id: uuidv4(), name: 'Patrícia Melo', email: 'patricia.melo@example.com' },
			{ id: uuidv4(), name: 'Rafael Torres', email: 'rafael.torres@example.com' },
			{ id: uuidv4(), name: 'Bruna Ferreira', email: 'bruna.ferreira@example.com' },
			{ id: uuidv4(), name: 'André Oliveira', email: 'andre.oliveira@example.com' },
			{ id: uuidv4(), name: 'Juliana Ramos', email: 'juliana.ramos@example.com' },
			{ id: uuidv4(), name: 'Gustavo Mendes', email: 'gustavo.mendes@example.com' },
			{ id: uuidv4(), name: 'Letícia Correia', email: 'leticia.correia@example.com' },
			{ id: uuidv4(), name: 'Rodrigo Batista', email: 'rodrigo.batista@example.com' }
		]
	};

	function removeUser(user_id: String) {
		console.log(user_id);
	}

	function removeManager(manager_id: String) {
		console.log(manager_id);
	}

	const saveChanges = () => {
		editing = false;
	};
</script>

<div class="container">
	<div class="w-full space-y-6">
		<Card.Root class="relative w-full overflow-hidden p-0">
			<div
				class="grid w-full md:grid-cols-[minmax(300px,1fr)_2fr] md:gap-4 lg:grid-cols-[minmax(400px,1fr)_3fr]"
			>
				<div class="relative aspect-[4/3] w-full md:aspect-auto">
					<img
						src={communityData.image}
						alt={`Image of ${community.name}`}
						class="h-full w-full object-cover"
					/>
				</div>

				<div class="flex w-full flex-col justify-center gap-6 p-6">
					<div>
						<h2 class="mb-1 text-sm font-medium text-muted-foreground">Name</h2>
						{#if editing}
							<Input
								type="text"
								placeholder="name"
								class="w-full rounded border border-gray-300 p-2 text-lg focus:ring-2 focus:ring-primary focus:outline-none"
								bind:value={community.name}
							/>
						{:else}
							<p class="text-2xl font-bold">{community.name}</p>
						{/if}
					</div>

					<div>
						<h2 class="mb-1 text-sm font-medium text-muted-foreground">Description</h2>
						{#if editing}
							<Input
								type="text"
								placeholder="description"
								class="w-full rounded border border-gray-300 p-2 text-lg focus:ring-2 focus:ring-primary focus:outline-none"
								bind:value={community.description}
							/>
						{:else}
							<p class="text-lg">{community.description}</p>
						{/if}
					</div>

					<div>
						<h2 class="mb-1 text-sm font-medium text-muted-foreground">Image</h2>
						{#if editing}
							<Input
								type="text"
								placeholder="image"
								class="w-full rounded border border-gray-300 p-2 text-lg focus:ring-2 focus:ring-primary focus:outline-none"
								bind:value={community.name}
							/>
						{:else}
							<p class="text-lg">{community.name}</p>
						{/if}
					</div>

					<div>
						<h2 class="mb-1 text-sm font-medium text-muted-foreground">Rule</h2>
						<p class="text-lg">{communityData.rule}</p>
					</div>
				</div>
			</div>

			<div class="absolute right-6 bottom-6 flex gap-2">
				{#if editing}
					<Button class="cursor-pointer" onclick={saveChanges}>
						<Save /> Save
					</Button>
					<Button class="cursor-pointer" onclick={() => (editing = false)}>
						Cancel
					</Button>
				{:else}
					<Button class="cursor-pointer" onclick={() => (editing = true)}>
						<Pencil /> Edit
					</Button>
				{/if}
			</div>
		</Card.Root>

		<div class="flex flex-col gap-6 lg:flex-row lg:items-start">
			<Card.Root class="w-full gap-4 lg:w-1/2">
				<Card.Header class="flex items-center justify-between border-b">
					<div class="flex items-center gap-2">
						<Users />
						<h2 class="text-lg font-semibold">Users</h2>
						<span class="text-sm text-muted-foreground">
							{communityData.users.length} members
						</span>
					</div>
					<Button class="cursor-pointer" onclick={() => (addUserDialogOpen = true)}>
						<UserPlus />
						Add User
					</Button>
				</Card.Header>

				<Card.Content>
					<ScrollArea class="max-h-97 w-full overflow-auto">
						<div class="space-y-3">
							{#each communityData.users as user}
								<div
									class="flex items-center justify-between gap-3 rounded-lg border bg-card p-3 transition-colors hover:bg-accent/50"
								>
									<div class="flex items-center gap-3">
										<Avatar.Root class="h-10 w-10">
											<Avatar.Fallback class="bg-primary/10 text-sm font-medium">
												{user.name.slice(0, 2).toUpperCase()}
											</Avatar.Fallback>
										</Avatar.Root>
										<span class="font-medium">{user.name}</span>
									</div>

									<Button
										variant="destructive"
										class="cursor-pointer text-sm font-medium"
										onclick={() => removeUser(user.id)}
									>
										<Trash_2 />
										Remove
									</Button>
								</div>
							{/each}
						</div>
					</ScrollArea>
				</Card.Content>
			</Card.Root>

			<Card.Root class="w-full gap-4 lg:w-1/2">
				<Card.Header class="flex items-center justify-between border-b">
					<div class="flex items-center gap-2">
						<Users />
						<h2 class="text-lg font-semibold">Managers</h2>
						<span class="text-sm text-muted-foreground">
							{communityData.users.length} members
						</span>
					</div>
					<Button class="cursor-pointer" onclick={() => (addManagerDialogOpen = true)}>
						<UserPlus />
						Add Manager
					</Button>
				</Card.Header>

				<Card.Content>
					<ScrollArea class="max-h-97 w-full overflow-auto">
						<div class="space-y-3">
							{#each communityData.users as manager}
								<div
									class="flex items-center justify-between gap-3 rounded-lg border bg-card p-3 transition-colors hover:bg-accent/50"
								>
									<div class="flex items-center gap-3">
										<Avatar.Root class="h-10 w-10">
											<Avatar.Fallback class="bg-primary/10 text-sm font-medium">
												{manager.name.slice(0, 2).toUpperCase()}
											</Avatar.Fallback>
										</Avatar.Root>
										<span class="font-medium">{manager.name}</span>
									</div>

									<Button
										variant="destructive"
										class="cursor-pointer text-sm font-medium"
										onclick={() => removeManager(manager.id)}
									>
										<Trash_2 />
										Remove
									</Button>
								</div>
							{/each}
						</div>
					</ScrollArea>
				</Card.Content>
			</Card.Root>
		</div>
	</div>
</div>

<AddMemberDialog
	bind:open={addUserDialogOpen}
	members={communityData.users}
	type="user"
	title="Find user..."
/>

<AddMemberDialog
	bind:open={addManagerDialogOpen}
	members={communityData.users}
	type="manager"
	title="Find manager..."
/>
