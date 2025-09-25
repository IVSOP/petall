<script lang="ts">
	import Users from '@lucide/svelte/icons/users';
	import UserLock from '@lucide/svelte/icons/user-lock';
	import Settings from '@lucide/svelte/icons/settings';
	import CircleDollarSign from '@lucide/svelte/icons/circle-dollar-sign';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as Card from '$lib/components/ui/card/index.js';

	const { community } = $props();
</script>

<div
	class="block transform rounded-b-xl transition-transform duration-300 hover:scale-102 hover:shadow-xl"
>
	<Card.Root class="gap-3 overflow-hidden rounded-xl pt-0">
		<Card.Header class="relative px-0 py-0">
			<div class="relative h-48 w-full overflow-hidden shadow-inner">
				<a href={`/community/${community.community.id}`}>
					<img
						src="https://media.istockphoto.com/id/1284536377/photo/solar-panels-on-field-in-summer-aerial-view-of-poland.jpg?s=612x612&w=0&k=20&c=CaH_vkg5gwZl8QzQTQYHhAqm0G5jEqhuuGR6V0l3wWc="
						alt={community.community.name}
						class="h-full w-full object-cover"
					/>
				</a>
				{#if community.role.includes('Manager')}
					<button class="absolute top-2 right-2 rounded-full bg-white/70 p-2 hover:bg-white">
						<a href={`/community/${community.community.id}/settings`}>
							<Settings />
						</a>
					</button>
				{/if}
			</div>
		</Card.Header>
		<Card.Content class="px-4">
			<div class="flex flex-col sm:justify-between xl:flex-row">
				<div class="truncate text-lg font-semibold">
					<a href={`/community/${community.community.id}`}>
						{community.community.name}
					</a>
				</div>
				<div class="mt-2 flex flex-wrap gap-1 sm:mt-0">
					{#if community.role == 'UserManager'}
						<Badge>User</Badge>
						<Badge>Manager</Badge>
					{:else}
						<Badge>{community.role}</Badge>
					{/if}
				</div>
			</div>
		</Card.Content>
		<Card.Content class="px-4">
			<div class="grid grid-cols-3 gap-4 text-center">
				<div class="flex flex-col items-center">
					<div
						class="mb-2 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100"
						title="Members"
					>
						<Users class="h-8 w-8" />
					</div>
					<p class="text-sm font-medium text-gray-700">13</p>
				</div>
				<div class="flex flex-col items-center">
					<div
						class="mb-2 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100"
						title="Managers"
					>
						<UserLock class="h-8 w-8" />
					</div>
					<p class="text-sm font-medium text-gray-700">5</p>
				</div>
				<div class="flex flex-col items-center">
					<div
						class="mb-2 flex h-16 w-16 items-center justify-center rounded-full bg-gray-100"
						title="Profit"
					>
						<CircleDollarSign class="h-8 w-8" />
					</div>
					<p class="text-sm font-medium text-gray-700">351 $</p>
				</div>
			</div>
		</Card.Content>

		<Card.Footer class="px-4">
			<p class="text-sm text-muted-foreground">
				Created: {new Date().toLocaleDateString()}
			</p>
		</Card.Footer>
	</Card.Root>
</div>
