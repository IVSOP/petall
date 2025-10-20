<script lang="ts">
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import type { LayoutProps } from './$types';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import CheckIcon from '@lucide/svelte/icons/check';
	import Plus from '@lucide/svelte/icons/plus';
	import ChevronsUpDownIcon from '@lucide/svelte/icons/chevrons-up-down';
	import { tick } from 'svelte';
	import * as Command from '$lib/components/ui/command/index.js';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import Settings from '@lucide/svelte/icons/settings';
	import { cn } from '$lib/utils.js';
	import { goto } from '$app/navigation';
	import type { Community } from '$lib';
	import Logout from '@tabler/icons-svelte/icons/logout';
	import Login from '@tabler/icons-svelte/icons/login';

	let { data, children }: LayoutProps = $props();

	let open = $state(false);
	let value = $state(data.communities.filter((c) => c.id === data.community.id)[0].name);

	let triggerRef = $state<HTMLButtonElement>(null!);

	const selectedValue = $derived(data.communities.find((c) => c.name === value)?.name);

	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef.focus();
		});
	}

	function selectCommunity(community: Community) {
		value = community.name;
		closeAndFocusTrigger();
		goto(`/community/${community.id}`);
	}

	function joinCommunity(community: Community) {
		console.log(community.name);
	}
</script>

<div class="flex flex-col gap-4 pb-4 md:flex-row md:justify-between">
	<div class="flex flex-col flex-row items-center gap-2">
		<Breadcrumb.Root>
			<Breadcrumb.List>
				<Breadcrumb.Item>
					<Breadcrumb.Page>{data.user?.name}</Breadcrumb.Page>
				</Breadcrumb.Item>
				<Breadcrumb.Separator />
				<Breadcrumb.Item>
					<Breadcrumb.Page>
						<Popover.Root bind:open>
							<Popover.Trigger bind:ref={triggerRef}>
								{#snippet child({ props })}
									<Button
										{...props}
										variant="outline"
										class="w-[210px] justify-between"
										role="combobox"
										aria-expanded={open}
									>
										{selectedValue || 'Select community...'}
										<ChevronsUpDownIcon class="opacity-50" />
									</Button>
								{/snippet}
							</Popover.Trigger>
							<Popover.Content class="w-[210px] p-0">
								<Command.Root>
									<Command.Input placeholder="Search community..." />
									<Command.List>
										<Command.Empty>No community found.</Command.Empty>
										<Command.Group value="communities">
											{#each data.communities as community}
												<Command.Item
													value={community.name}
													onSelect={() => selectCommunity(community)}
												>
													<CheckIcon class={cn(value !== community.name && 'text-transparent')} />
													{community.name}
												</Command.Item>
											{/each}
										</Command.Group>
									</Command.List>
								</Command.Root>
							</Popover.Content>
						</Popover.Root>
					</Breadcrumb.Page>
				</Breadcrumb.Item>
			</Breadcrumb.List>
		</Breadcrumb.Root>

		<div class="flex gap-2">
			<Button href="/community/{data.community.id}/settings">
				<Settings />
			</Button>
			<Button href="/community/{data.community.id}/leave">
				<Logout />
			</Button>
		</div>
	</div>

	<div class="flex flex-col gap-2 md:flex-row">
		<Popover.Root>
			<Popover.Trigger class={buttonVariants()}>
				<Login />
				Join Community
			</Popover.Trigger>
			<Popover.Content class="w-[190px] p-0">
				<Command.Root>
					<Command.Input placeholder="Find community..." />
					<Command.List>
						<Command.Empty>No community found.</Command.Empty>
						<Command.Group value="communities">
							<ScrollArea class="max-h-[10rem]">
								{#each data.communities as community}
									<Command.Item value={community.name} onSelect={() => joinCommunity(community)}>
										{community.name}
									</Command.Item>
								{/each}
							</ScrollArea>
						</Command.Group>
					</Command.List>
				</Command.Root>
			</Popover.Content>
		</Popover.Root>

		<Button href="/community/new">
			<Plus />
			New Community
		</Button>
	</div>
</div>

{@render children()}
