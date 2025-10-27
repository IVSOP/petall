<script lang="ts">
	import '../../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as NavigationMenu from '$lib/components/ui/navigation-menu/index.js';
	import { navigationMenuTriggerStyle } from '$lib/components/ui/navigation-menu/navigation-menu-trigger.svelte';
	import type { LayoutProps } from './$types';
	import Avatar from '$lib/components/Avatar.svelte';

	let { data, children }: LayoutProps = $props();
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
							<Button
								href="/"
								variant="ghost"
								class="text-primary-background cursor-pointer items-end py-0 text-3xl leading-none font-semibold transition-opacity hover:opacity-70"
							>
								PetAll
							</Button>
						</NavigationMenu.Item>
						<NavigationMenu.Item>
							<Button
								href="/admin"
								variant="ghost"
								class="cursor-pointer items-end py-0 text-lg leading-none font-semibold transition-opacity hover:opacity-70"
							>
								Admin view
							</Button>
						</NavigationMenu.Item>
						<NavigationMenu.Item>
							<Button
								href="/"
								variant="ghost"
								class="cursor-pointer items-end py-0 text-lg leading-none font-semibold transition-opacity hover:opacity-70"
							>
								User view
							</Button>
						</NavigationMenu.Item>
					</NavigationMenu.List>
				</NavigationMenu.Root>

				{#if data.user}
					<Avatar name={data.user.name} />
				{/if}
			</div>
		</div>
	</header>
	<main class="flex-1">
		<div class="container mx-auto">
			<div class="px-4 py-5">
				{@render children()}
			</div>
		</div>
	</main>
</div>
