<script lang="ts">
	import '../../app.css';
	import favicon from '$lib/assets/favicon.svg';
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
	<header class="w-full border-b">
		<div class="container mx-auto flex h-14 items-center px-4">
			<div class="flex items-center gap-6">
				<a href="/" class="flex items-center gap-2 font-semibold text-foreground">
					<img src={favicon} alt="PeTall" class="h-6 w-6" />
					<span class="text-2xl hover:opacity-70">PeTall</span>
				</a>

				<nav aria-label="Main">
					<NavigationMenu.Root class="flex-none justify-start">
						<NavigationMenu.List>
							{#if data.user?.canAccessAdminView}
								<NavigationMenu.Item>
									<NavigationMenu.Link href="/admin" class={navigationMenuTriggerStyle()}>
										Admin View
									</NavigationMenu.Link>
								</NavigationMenu.Item>
							{/if}
						</NavigationMenu.List>
					</NavigationMenu.Root>
				</nav>
			</div>

			<div class="ml-auto flex items-center gap-3">
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