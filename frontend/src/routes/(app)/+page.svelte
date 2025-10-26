<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
</script>

<div class="flex flex-col items-center justify-center space-y-2 py-12 text-center">
	<p class="text-lg font-medium text-primary sm:text-2xl">
		You don't have records of any community
	</p>
	<p class="text-muted-foreground">Wait for an administrator to add you to a community.</p>

	<Button class="mt-4" variant="destructive" size="lg" href="/community/new">
		As this is a demo build, you can create a community here
	</Button>
</div>
