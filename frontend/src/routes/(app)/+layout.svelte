<script lang="ts">
	import '../../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import * as NavigationMenu from '$lib/components/ui/navigation-menu/index.js';
	import type { LayoutProps } from './$types';
	import Avatar from '$lib/components/Avatar.svelte';
	import { Button } from '$lib/components/ui/button/index.js';

	let { data, children }: LayoutProps = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="flex min-h-screen flex-col bg-background">
	<header class="w-full border-b">
		<div class="container mx-auto px-4 py-2">
			<div class="flex justify-between">
				<NavigationMenu.Root>
					<NavigationMenu.List>
						<NavigationMenu.Item>
							<Button
								href="/"
								variant="ghost"
								class="cursor-pointer items-end bg-transparent pb-0 pl-0 text-3xl leading-none font-semibold transition-opacity hover:bg-transparent hover:opacity-70"
							>
								<img src={favicon} alt="PeTall" class="h-8 w-8" />
								PeTall
							</Button>
						</NavigationMenu.Item>
						{#if data.user?.canAccessAdminView}
							<NavigationMenu.Item>
								<Button
									href="/admin"
									variant="ghost"
									class="cursor-pointer items-end bg-transparent pb-1 text-lg leading-none font-semibold transition-opacity hover:bg-transparent hover:opacity-70"
								>
									Admin view
								</Button>
							</NavigationMenu.Item>
						{/if}
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
