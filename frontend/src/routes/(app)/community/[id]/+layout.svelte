<script lang="ts">
	import type { LayoutProps } from './$types';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import CheckIcon from '@lucide/svelte/icons/check';
	import ChevronsUpDownIcon from '@lucide/svelte/icons/chevrons-up-down';
	import { tick } from 'svelte';
	import * as Command from '$lib/components/ui/command/index.js';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { cn } from '$lib/utils.js';
	import { goto } from '$app/navigation';
	import type { Community } from '$lib';

	let { data, children }: LayoutProps = $props();

	let open = $state(false);
	let triggerRef = $state<HTMLButtonElement>(null!);

	let value = $state(data.community.name);
	const selectedCommunity = $derived(data.communities.find((c) => c.name === value));
	const selectedValue = $derived(selectedCommunity?.name);
	const selectedIsPresent = $derived(selectedCommunity?.isPresent);

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
</script>

<div class="flex flex-col gap-4 pb-4 md:flex-row md:justify-between">
	<div class="flex flex-row items-center gap-2">
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
										class="w-auto max-w-full min-w-[210px] justify-between"
										role="combobox"
										aria-expanded={open}
									>
										<span class="flex items-center gap-2">
											{selectedValue || 'Select community...'}
											{#if selectedIsPresent === false}
												<Badge variant="outline" class="text-xs">Not present</Badge>
											{/if}
										</span>
										<ChevronsUpDownIcon class="opacity-50" />
									</Button>
								{/snippet}
							</Popover.Trigger>
							<Popover.Content class="w-auto max-w-full min-w-[210px] p-0">
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
													{#if !community.isPresent}
														<Badge variant="outline" class="ml-auto">Not present</Badge>
													{/if}
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
	</div>
</div>

{@render children()}
