<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import * as NavigationMenu from '$lib/components/ui/navigation-menu/index.js';
	import Avatar from '../components/Avatar.svelte';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="flex min-h-screen flex-col bg-background">
	<div class="border-y bg-muted/60">
		<div class="container mx-auto px-3 py-2">
			<div class="flex justify-between">
				<NavigationMenu.Root>
					<NavigationMenu.List>
						<NavigationMenu.Item>
							<NavigationMenu.Link href="/">PETALL</NavigationMenu.Link>
						</NavigationMenu.Item>
					</NavigationMenu.List>
				</NavigationMenu.Root>
				<NavigationMenu.Root>
					<NavigationMenu.List>
						<NavigationMenu.Item>
							<Avatar image="https://github.com/DigoqueDigo.png" />
						</NavigationMenu.Item>
					</NavigationMenu.List>
				</NavigationMenu.Root>
				<!-- <button
          type="button"
          class="flex cursor-pointer items-center gap-3 transition-opacity hover:opacity-80"
        </button> -->
			</div>
		</div>
	</div>

	<main class="flex-1">
		<div class="container mx-auto">
			<div class="px-4 py-5">
				{@render children()}
			</div>
		</div>
	</main>
</div>
