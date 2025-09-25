<script lang="ts">
	import type { PageProps } from './$types';
	import AlertCircleIcon from '@lucide/svelte/icons/alert-circle';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Alert from '$lib/components/ui/alert/index.js';

	const { data }: PageProps = $props();
</script>

<div class="container">
	<div
		class="text-w-full mb-4 rounded-md border bg-muted px-4 py-2 text-lg font-semibold md:text-xl"
	>
		{data.community.name}
	</div>

	<div class="flex flex-col gap-6 py-0 lg:flex-row lg:items-start">
		<div>
			<button
				class="relative w-full cursor-pointer overflow-hidden rounded-lg border-2 focus:outline-none"
				aria-label="Change community picture"
				type="button"
			>
				<img
					src="https://media.istockphoto.com/id/1284536377/photo/solar-panels-on-field-in-summer-aerial-view-of-poland.jpg?s=612x612&w=0&k=20&c=CaH_vkg5gwZl8QzQTQYHhAqm0G5jEqhuuGR6V0l3wWc="
					alt="Solar panels on a field in summer"
					class="h-auto w-full object-cover ring-2 ring-transparent transition hover:ring-gray-300"
				/>
				<div
					class="absolute inset-0 flex items-center justify-center rounded-lg bg-black/40 text-white opacity-0 transition hover:opacity-100"
				>
					Change
				</div>
			</button>
		</div>

		<div class="grid flex-1 gap-4">
			<div class="grid grid-cols-8 items-center gap-4">
				<Label for="name" class="font-sm text-right">Name</Label>
				<Input
					id="name"
					value={data.community.name}
					class="col-span-7"
					placeholder="Enter community name"
				/>
			</div>
		</div>
	</div>

	<div
		class="text-w-full my-4 rounded-md border bg-muted px-4 py-2 text-lg font-semibold md:text-xl"
	>
		Manage access
	</div>

	<div
		class="text-w-full my-4 rounded-md border bg-destructive px-4 py-2 text-lg font-semibold text-white md:text-xl"
	>
		Danger zone
	</div>

	<div class="space-y-4">

		<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
			<Alert.Root variant="destructive">
				<AlertCircleIcon />
				<Alert.Title>
					Please be aware of the implications of
					<span class="font-extrabold underline">
						leaving {data.community.name}
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
						deleting {data.community.name}
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
		</div>

		<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
			<Button
				variant="secondary"
				class="w-full rounded-md border border-red-600 bg-white px-4 py-2 text-red-600 hover:bg-red-50 cursor-pointer"
			>
				Leave Community
			</Button>
			<Button variant="destructive" class="w-full rounded-md px-4 py-2 cursor-pointer">
                Delete Community
            </Button>
		</div>
	</div>
</div>
