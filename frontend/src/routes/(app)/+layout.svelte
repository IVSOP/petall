<script lang="ts">
	import '../../app.css';
	import type { LayoutProps } from '../$types';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import favicon from '$lib/assets/favicon.svg';
	import * as NavigationMenu from '$lib/components/ui/navigation-menu/index.js';
	import Avatar from '../../components/Avatar.svelte';

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
							<a
								class="text-primary-background cursor-pointer items-center gap-3 text-3xl font-semibold transition-opacity hover:opacity-70"
								href="/"
							>
								PeTall
							</a>
						</NavigationMenu.Item>
					</NavigationMenu.List>
				</NavigationMenu.Root>
				<Avatar participant={data.participant} image={data.image} />
			</div>
		</div>
	</div>
	<main class="flex-1">
		<div class="container mx-auto">
			<div class="px-4 py-5">
				<Toaster />
				{@render children()}
			</div>
		</div>
	</main>
</div>
