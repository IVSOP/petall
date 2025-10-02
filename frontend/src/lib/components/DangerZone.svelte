<script lang="ts">
	import AlertCircleIcon from '@lucide/svelte/icons/alert-circle';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Alert from '$lib/components/ui/alert/index.js';

	const { community } = $props();
</script>

<div class="grid gap-4">
	<div
		class="text-w-full rounded-md border border-destructive bg-destructive px-4 py-2 text-lg font-semibold text-white md:text-xl"
	>
		Danger zone
	</div>
	<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
		<Alert.Root variant="destructive">
			<AlertCircleIcon />
			<Alert.Title>
				Please be aware of the implications of
				<span class="font-extrabold underline">
					leaving {community.name}
				</span>
				community.
			</Alert.Title>
			<Alert.Description>
				<ul class="list-inside list-disc text-sm">
					<li>Member privileges will be lost</li>
					<li>You'll no longer have access to community content</li>
					<li>Make sure responsibilities are transferred</li>
				</ul>
			</Alert.Description>
		</Alert.Root>
		<Alert.Root variant="destructive">
			<AlertCircleIcon />
			<Alert.Title>
				Please be aware of the implications of
				<span class="font-extrabold underline">
					deleting {community.name}
				</span>
				community.
			</Alert.Title>
			<Alert.Description>
				<ul class="list-inside list-disc text-sm">
					<li>Members will lose access</li>
					<li>All content will be permanently removed</li>
					<li>This action cannot be undone</li>
				</ul>
			</Alert.Description>
		</Alert.Root>
		<Button
			variant="secondary"
			class="w-full cursor-pointer rounded-md border border-red-600 bg-white px-4 py-2 text-red-600 hover:bg-red-50"
		>
			Leave Community
		</Button>
		<Button variant="destructive" class="w-full cursor-pointer rounded-md px-4 py-2">
			Delete Community
		</Button>
	</div>
</div>
