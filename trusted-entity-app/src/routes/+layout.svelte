<script lang="ts">
	import '../app.css';
	import * as NavigationMenu from '$lib/components/ui/navigation-menu/index.js';
	import type { LayoutProps } from './$types';
	import { Button } from '$lib/components/ui/button/index.js';

	let { data, children }: LayoutProps = $props();
</script>

<div class="flex min-h-screen flex-col bg-background">
	<header class="w-full border-b">
		<div class="container mx-auto px-4 py-2">
			<div class="flex justify-between">
				<NavigationMenu.Root>
					<NavigationMenu.List>
						<NavigationMenu.Item>
							<Button variant="ghost" class="pl-0 text-3xl font-semibold hover:bg-transparent">
								<img src="/favicon.ico" alt="PeTall" class="h-8 w-11" />
								PeTall
							</Button>
						</NavigationMenu.Item>
					</NavigationMenu.List>
				</NavigationMenu.Root>
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
