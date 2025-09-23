<script lang="ts">
	import SquarePlus from '@lucide/svelte/icons/square-plus';

	import { Button, buttonVariants } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
</script>

<Dialog.Root>
	<Dialog.Trigger class="focus:outline-none">
		<Button
			class="order-1 w-full cursor-pointer border-none bg-green-600 text-white hover:bg-green-700 md:order-none"
		>
			<SquarePlus />
			New Community
		</Button>
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
					class="relative w-full  cursor-pointer rounded-lg focus:outline-none"
					aria-label="Change community picture"
					type="button"
				>
					<img
						src="https://media.istockphoto.com/id/1284536377/photo/solar-panels-on-field-in-summer-aerial-view-of-poland.jpg?s=612x612&w=0&k=20&c=CaH_vkg5gwZl8QzQTQYHhAqm0G5jEqhuuGR6V0l3wWc="
						alt="Solar panels on a field in summer"
						class="h-auto w-full rounded-lg object-cover ring-2 ring-transparent transition hover:ring-gray-300"
					/>
					<div
						class="absolute inset-0 flex items-center justify-center rounded-lg bg-black/40 text-white opacity-0 transition hover:opacity-100"
					>
						Change
					</div>
				</button>
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="community-name" class="text-right">Name</Label>
				<Input id="community-name" value="" class="col-span-3" placeholder="Enter community name" />
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="description" class="text-right">Description</Label>
				<Input
					id="description"
					value=""
					class="col-span-3"
					placeholder="Enter a short description"
				/>
			</div>
		</div>
		<Dialog.Footer>
			<Button type="submit">Save Community</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
