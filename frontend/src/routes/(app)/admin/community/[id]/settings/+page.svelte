<script lang="ts">
	import * as Card from '$lib/components/ui/card/index.js';
	import AddUserDialog from './AddUserDialog.svelte';
	import ManageMembers from './ManageUsers.svelte';
	import Zap from '@lucide/svelte/icons/zap';
	import ErrorDialog from '$lib/components/ErrorDialog.svelte';
	import type { PageProps } from './$types';

	const { data, form }: PageProps = $props();

	console.log(data.admin_community_info);

	let addUserDialogOpen: boolean = $state(false);
	let addManagerDialogOpen: boolean = $state(false);
</script>

<svelte:head>
	<title>{data.admin_community_info.community.name} - Settings</title>
</svelte:head>

<div class="container">
	<div class="w-full space-y-6">
		<Card.Root class="w-full overflow-hidden p-0">
			<div
				class="grid w-full md:grid-cols-[minmax(300px,1fr)_2fr] md:gap-4 lg:grid-cols-[minmax(400px,1fr)_3fr]"
			>
				<div class="aspect-[4/3] w-full md:aspect-auto">
					{#if data.admin_community_info.community.image}
						<img
							src={data.admin_community_info.community.image}
							alt={`Image of ${data.admin_community_info.community.name}`}
							class="h-full w-full object-cover"
						/>
					{:else}
						<div class="flex h-full w-full items-center justify-center bg-muted">
							<div class="flex flex-col items-center gap-3">
								<div class="rounded-full bg-background p-4 shadow-sm">
									<Zap class="h-10 w-10 text-muted-foreground" />
								</div>
								<p class="text-sm text-muted-foreground">No image available</p>
							</div>
						</div>
					{/if}
				</div>

				<div class="flex w-full flex-col justify-center gap-6 p-6">
					<div>
						<h2 class="mb-1 text-sm font-medium text-muted-foreground">Name</h2>
						<p class="text-xl font-bold">{data.admin_community_info.community.name}</p>
					</div>

					<div>
						<h2 class="mb-1 text-sm font-medium text-muted-foreground">Description</h2>
						<p class="text-md">{data.admin_community_info.community.description}</p>
					</div>

					<div>
						<h2 class="mb-1 text-sm font-medium text-muted-foreground">Rule</h2>
						<p class="text-md">n/p</p>
					</div>
				</div>
			</div>
		</Card.Root>

		{#if form?.error}
			<div>
				<ErrorDialog>
					{form?.error}
				</ErrorDialog>
			</div>
		{/if}

		<div class="flex flex-col gap-6 lg:flex-row lg:items-start">
			<ManageMembers
				bind:open={addUserDialogOpen}
				users={data.admin_community_info.users}
				admins={data.admin_community_info.admins}
				user_type="user"
				title="Users"
			/>
			<ManageMembers
				bind:open={addManagerDialogOpen}
				users={data.admin_community_info.managers.concat(data.admin_community_info.admins)}
				admins={data.admin_community_info.admins}
				user_type="manager"
				title="Managers"
			/>
		</div>
	</div>
</div>

<AddUserDialog bind:open={addUserDialogOpen} user_type="user" title="Add User" />

<AddUserDialog bind:open={addManagerDialogOpen} user_type="manager" title="Add Manager" />
