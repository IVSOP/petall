<script lang="ts">
	import '../../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import * as NavigationMenu from '$lib/components/ui/navigation-menu/index.js';
	import type { LayoutProps } from './$types';
	import Avatar from '$lib/components/Avatar.svelte';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';

	let { data, children }: LayoutProps = $props();

	import CheckIcon from '@lucide/svelte/icons/check';
	import ChevronsUpDownIcon from '@lucide/svelte/icons/chevrons-up-down';
	import { tick } from 'svelte';
	import * as Command from '$lib/components/ui/command/index.js';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { cn } from '$lib/utils.js';

	let open = $state(false);
	let value = $state('');
	let triggerRef = $state<HTMLButtonElement>(null!);

	const selectedValue = $derived(data.communities.find((c) => c.name === value)?.name);

	// We want to refocus the trigger button when the user selects
	// an item from the list so users can continue navigating the
	// rest of the form with the keyboard.
	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef.focus();
		});
	}
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="flex min-h-screen flex-col bg-background">
	<div class="border-y bg-muted/100">
		<div class="container mx-auto px-3 py-2">
			<div class="flex justify-between">
				<NavigationMenu.Root>
					<NavigationMenu.List>
						<NavigationMenu.Item>
							<a
								class="text-primary-background cursor-pointer items-center gap-3 text-3xl font-semibold transition-opacity hover:opacity-70"
								href="/"
							>
								PeTall
							</a>
						</NavigationMenu.Item>
					</NavigationMenu.List>
				</NavigationMenu.Root>
				{#if data.user}
					<Avatar name={data.user.name} />
				{/if}
			</div>
		</div>
	</div>
	<main class="flex-1">
		<div class="container mx-auto">
			<div class="px-4 py-5">
				<div class="pb-4">
					<Breadcrumb.Root>
						<Breadcrumb.List>
							<Breadcrumb.Item>
								<Breadcrumb.Link href="/">{data.user.name}</Breadcrumb.Link>
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
															<a
																href="/community/{community.id}"
																class="text-inherit no-underline"
															>
																<Command.Item
																	value={community.name}
																	onSelect={() => {
																		value = community.name;
																		closeAndFocusTrigger();
																	}}
																>
																	<CheckIcon
																		class={cn(value !== community.name && 'text-transparent')}
																	/>
																	{community.name}
																</Command.Item>
															</a>
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
				{@render children()}
			</div>
		</div>
	</main>
</div>
