<script lang="ts">
    import type { LayoutProps } from './$types';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
    import * as ButtonGroup from "$lib/components/ui/button-group/index.js";
	import CheckIcon from '@lucide/svelte/icons/check';
	import ChevronsUpDownIcon from '@lucide/svelte/icons/chevrons-up-down';
	import { tick } from 'svelte';
	import * as Command from '$lib/components/ui/command/index.js';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import Settings from '@lucide/svelte/icons/settings';
	import { cn } from '$lib/utils.js';
	import { goto } from '$app/navigation';
	import type { Community } from '$lib';

	let { data, children }: LayoutProps = $props();

	let open = $state(false);
	let value = $state(data.communities.filter((c) => c.id === data.communityId)[0].name);
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
</script>

<div class="pb-4 flex gap-2.5">
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
									class="w-[200px] justify-between"
									role="combobox"
									aria-expanded={open}
								>
									{selectedValue || 'Select community...'}
									<ChevronsUpDownIcon class="opacity-50" />
								</Button>
							{/snippet}
						</Popover.Trigger>
						<Popover.Content class="w-[200px] p-0">
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
			<Breadcrumb.Separator />
		</Breadcrumb.List>
	</Breadcrumb.Root>
    <ButtonGroup.Root>
		<Button
			href="/community/{data.communityId}/settings"
			variant="secondary">
				<Settings />
		</Button>
		<Button
			variant="secondary"
			href="/community/new">
				New Community
		</Button>		
    </ButtonGroup.Root>
</div>

{@render children()}
