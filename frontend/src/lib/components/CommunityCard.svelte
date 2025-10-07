<script lang="ts">
	import Users from '@lucide/svelte/icons/users';
	import UserLock from '@lucide/svelte/icons/user-lock';
	import CircleDollarSign from '@lucide/svelte/icons/circle-dollar-sign';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import type { CommunityGetCommunitiesResponse } from '$lib/api/community';

	const { community }: { community: CommunityGetCommunitiesResponse } = $props();

	console.log(community);
</script>

<a href={`/community/${community.id}`}>
	<div
		class="block transform rounded-xl transition-transform duration-300 hover:scale-[1.01] hover:shadow-lg"
	>
		<Card.Root class="flex flex-col gap-0 overflow-hidden rounded-xl py-0 shadow-md md:flex-row">
			<div class="relative px-0 py-0 md:w-1/3 lg:w-1/4">
				<div class="relative h-48 w-full shadow-inner">
					<img
						src="https://images.jdmagicbox.com/rep/b2b/wall-paper/wall-paper-11.jpg"
						alt={community.name}
						class="h-full w-full object-cover"
					/>
				</div>
			</div>

			<div class="flex flex-col justify-between p-4 md:w-2/3 lg:w-3/4">
				<Card.Content class="flex flex-col justify-between gap-2 px-2">
					<div class="flex flex-col sm:flex-row sm:justify-between">
						<h2 class="truncate text-xl leading-tight font-semibold hover:underline">
							{community.name}
						</h2>
						<div class="mt-2 flex flex-wrap gap-1 sm:mt-0">
							<Badge variant="secondary" class=" text-sm">User</Badge>
							<Badge variant="secondary" class=" text-sm">Manager</Badge>
						</div>
					</div>

					<p class="text-md line-clamp-3 text-justify text-gray-400">
						It is a long established fact that a reader will be distracted by the readable content
						of a page when looking at its layout. The point of using Lorem Ipsum is that it has a
						more-or-less normal distribution of letters
					</p>
				</Card.Content>

				<Card.Footer
					class="text-md mt-3 flex flex-wrap items-center justify-between px-2 text-gray-400"
				>
					<div class="flex items-center gap-4">
						<div class="flex items-center gap-1" title="Members">
							<Users class="h-4 w-4" /> 13
						</div>
						<div class="flex items-center gap-1" title="Managers">
							<UserLock class="h-4 w-4" /> 5
						</div>
						<div class="flex items-center gap-1" title="Profit">
							<CircleDollarSign class="h-4 w-4" /> 351 $
						</div>
					</div>
					<p>07/08/2024</p>
				</Card.Footer>
			</div>
		</Card.Root>
	</div>
</a>
