<script lang="ts">
	import '../../app.css';
	import * as NavigationMenu from '$lib/components/ui/navigation-menu/index.js';
	import type { LayoutProps } from './$types';
	import Avatar from '$lib/components/Avatar.svelte';
	import { Button } from '$lib/components/ui/button/index.js';
	import Leaf from '@tabler/icons-svelte/icons/leaf';

	let { data, children }: LayoutProps = $props();
</script>

<div class="flex min-h-screen flex-col bg-background">
	<header class="w-full border-b">
		<div class="container mx-auto px-4 py-2">
			<div class="flex justify-between">
				<NavigationMenu.Root>
					<NavigationMenu.List>
						<NavigationMenu.Item>
							<a href="/" class="cursor-pointer flex items-center hover:bg-transparent hover:opacity-70">
								<Leaf />
								<Button
									variant="ghost"
									class="cursor-pointer pl-1 text-3xl font-semibold transition-opacity hover:bg-transparent"
								>
									EnerJizz
								</Button>
							</a>
						</NavigationMenu.Item>
						{#if data.user?.canAccessAdminView}
							<NavigationMenu.Item>
								<Button
									href="/admin"
									variant="ghost"
									class="cursor-pointer bg-transparent text-lg font-semibold transition-opacity hover:bg-transparent hover:opacity-70"
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
