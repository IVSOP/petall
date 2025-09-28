<script lang="ts">
	import SquarePlus from '@lucide/svelte/icons/square-plus';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Button, buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
</script>

<Dialog.Root>
	<Dialog.Trigger
		class={`${buttonVariants({ variant: 'outline' })} w-full bg-green-600 text-sm text-white hover:bg-green-700 hover:text-white focus:outline-none md:text-base`}
	>
		<SquarePlus />
		New Community
	</Dialog.Trigger>
	<Dialog.Content class="sm:max-w-[500px]">
		<Dialog.Header>
			<Dialog.Title>Create a New Community</Dialog.Title>
			<Dialog.Description>
				Fill in the details to create your community. Click save when you're done.
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4">
			<div class="flex justify-center">
				<button
					class="relative w-full cursor-pointer overflow-hidden rounded-lg border focus:outline-none"
					aria-label="Change community picture"
					type="button"
				>
					<img
						src="https://images.jdmagicbox.com/rep/b2b/wall-paper/wall-paper-11.jpg"
						alt="Solar panels on a field in summer"
						class="h-48 w-full object-cover ring-2 ring-transparent transition hover:ring-gray-300"
					/>
					<div
						class="absolute inset-0 flex items-center justify-center rounded-lg bg-black/40 text-white opacity-0 transition hover:opacity-100"
					>
						Change
					</div>
				</button>
			</div>
			<div class="grid gap-2">
				<Label for="community-name" class="text-right">Name</Label>
				<Input id="community-name" class="text-sm" placeholder="Enter community name" />
			</div>
			<div class="grid gap-2">
				<Label for="calculation-tule" class="text-right">Rule</Label>
				<Input id="calculation-rule" class="text-sm" placeholder="Enter calculation rule" />
			</div>
			<div class="grid gap-2">
				<Label for="description" class="text-righ">Description</Label>
				<Textarea id="description" class="h-0 text-sm" placeholder="Enter community decription" />
			</div>
		</div>
		<Dialog.Footer>
			<Button type="submit" class="w-full">Save Community</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
