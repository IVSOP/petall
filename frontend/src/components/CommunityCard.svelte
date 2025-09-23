<script lang="ts">
	import Users from '@lucide/svelte/icons/users';
	import UserLock from '@lucide/svelte/icons/user-lock';
	import CircleDollarSign from '@lucide/svelte/icons/circle-dollar-sign';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as Card from '$lib/components/ui/card/index.js';

	const { community } = $props();
</script>

<a
	href={`/community/${community.id}`}
	class="block transform rounded-b-xl transition-transform duration-300 hover:scale-102 hover:shadow-xl"
>
	<Card.Root class="gap-3 overflow-hidden rounded-xl pt-0">
		<Card.Header class="px-0 py-0">
			<div
				class="flex h-48 w-full items-center justify-center overflow-hidden rounded-t-xl shadow-inner"
			>
				<img
					src="https://media.istockphoto.com/id/1284536377/photo/solar-panels-on-field-in-summer-aerial-view-of-poland.jpg?s=612x612&w=0&k=20&c=CaH_vkg5gwZl8QzQTQYHhAqm0G5jEqhuuGR6V0l3wWc="
					alt={community.name}
					class="h-full w-full object-cover"
				/>
			</div>
		</Card.Header>
		<Card.Content class="px-4 ">
			<div class="flex justify-between">
				<div class="truncate text-lg font-semibold text-gray-900">
					{community.name}
				</div>
				<Badge>User</Badge>
			</div>
		</Card.Content><Card.Content class="px-4">
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
</a>
