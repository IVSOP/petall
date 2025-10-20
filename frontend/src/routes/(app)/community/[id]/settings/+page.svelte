<script lang="ts">
	import type { PageProps } from './$types';
	import * as Card from '$lib/components/ui/card/index.js';
	import * as Avatar from '$lib/components/ui/avatar/index.js';
	import Users from '@lucide/svelte/icons/users';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';

	const communityData = {
		name: 'Creative Community',
		rule: 'Be respectful and creative',
		image:
			'https://media.istockphoto.com/id/814423752/photo/eye-of-model-with-colorful-art-make-up-close-up.jpg?s=612x612&w=0&k=20&c=l15OdMWjgCKycMMShP8UK94ELVlEGvt7GmB_esHWPYE=',
		participants: [
			{ id: 1, name: 'Chico2', avatar: null },
			{ id: 2, name: 'Pedro2', avatar: '/placeholder.svg?height=40&width=40' },
			{ id: 3, name: 'Digo3', avatar: '/placeholder.svg?height=40&width=40' },
			{ id: 4, name: 'IVSOP3', avatar: null },
			{ id: 5, name: 'Chico3', avatar: null },
			{ id: 6, name: 'Pedro3', avatar: '/placeholder.svg?height=40&width=40' },
			{ id: 4, name: 'IVSOP3', avatar: null },
			{ id: 5, name: 'Chico3', avatar: null },
			{ id: 6, name: 'Pedro3', avatar: '/placeholder.svg?height=40&width=40' },
			{ id: 1, name: 'Chico2', avatar: null },
			{ id: 2, name: 'Pedro2', avatar: '/placeholder.svg?height=40&width=40' },
			{ id: 3, name: 'Digo3', avatar: '/placeholder.svg?height=40&width=40' },
			{ id: 4, name: 'IVSOP3', avatar: null },
			{ id: 5, name: 'Chico3', avatar: null },
			{ id: 6, name: 'Pedro3', avatar: '/placeholder.svg?height=40&width=40' },
			{ id: 4, name: 'IVSOP3', avatar: null },
			{ id: 5, name: 'Chico3', avatar: null },
			{ id: 6, name: 'Pedro3', avatar: '/placeholder.svg?height=40&width=40' }
		]
	};

	const { data }: PageProps = $props();
</script>

<div class="container">
	<div class="w-full space-y-6">
		<Card.Root class="w-full overflow-hidden p-0">
			<div
				class="grid w-full md:grid-cols-[minmax(300px,1fr)_2fr] md:gap-4 lg:grid-cols-[minmax(400px,1fr)_3fr]"
			>
				<div class="relative aspect-[4/3] w-full md:aspect-auto">
					<img
						src={communityData.image}
						alt={`Image of ${data.community.name}`}
						class="h-full w-full object-cover"
					/>
				</div>

				<div class="flex w-full flex-col justify-center gap-6 p-6">
					<div>
						<h2 class="mb-1 text-sm font-medium text-muted-foreground">Community Name</h2>
						<p class="text-2xl font-bold">{data.community.name}</p>
					</div>

					<div>
						<h2 class="mb-1 text-sm font-medium text-muted-foreground">Community Description</h2>
						<p class="text-lg">{data.community.description}</p>
					</div>

					<div>
						<h2 class="mb-1 text-sm font-medium text-muted-foreground">Community Rule</h2>
						<p class="text-lg">{communityData.rule}</p>
					</div>
				</div>
			</div>
		</Card.Root>

		<Card.Root class="w-full">
			<Card.Header>
				<Card.Title class="flex items-center gap-2">
					<Users class="h-5 w-5" />
					Participants
					<span class="ml-auto text-base font-normal text-muted-foreground">
						{communityData.participants.length} members
					</span>
				</Card.Title>
			</Card.Header>

			<Card.Content>
				<ScrollArea class="max-h-97 w-full overflow-auto">
					<div class="space-y-3">
						{#each communityData.participants as participant}
							<div
								class="flex items-center gap-3 rounded-lg border bg-card p-3 transition-colors hover:bg-accent/50"
							>
								<Avatar.Root class="h-10 w-10">
									<Avatar.Fallback class="bg-primary/10 text-sm font-medium">
										{participant.name.slice(0, 2).toUpperCase()}
									</Avatar.Fallback>
								</Avatar.Root>
								<span class="font-medium">{participant.name}</span>
							</div>
						{/each}
					</div>
				</ScrollArea>
			</Card.Content>
		</Card.Root>
	</div>
</div>
