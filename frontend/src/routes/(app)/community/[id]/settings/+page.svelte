<script lang="ts">
	import type { PageProps } from './$types';
	import { v4 as uuidv4 } from 'uuid';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import AddMemberDialog from './AddMemberDialog.svelte';
	import type { Community } from '$lib';
	import Pencil from '@lucide/svelte/icons/pencil';
	import Save from '@lucide/svelte/icons/save';
	import X from '@lucide/svelte/icons/x';
	import ManageMembers from './ManageMembers.svelte';

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
						<X /> Cancel
					</Button>
				{:else}
					<Button class="cursor-pointer" onclick={() => (editing = true)}>
						<Pencil /> Edit
					</Button>
				{/if}
			</div>
		</Card.Root>

		<div class="flex flex-col gap-6 lg:flex-row lg:items-start">
			<ManageMembers 
				bind:open={addUserDialogOpen}
				members={communityData.users}
				type="user"
				title="Users"
			/>
			<ManageMembers 
				bind:open={addManagerDialogOpen}
				members={communityData.users}
				type="manager"
				title="Managers"
			/>
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
