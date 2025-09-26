<script lang="ts">
	import type { PageProps } from './$types';
	import Info from '@lucide/svelte/icons/info';
	import Search from '@lucide/svelte/icons/search';
	import AlertCircleIcon from '@lucide/svelte/icons/alert-circle';
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Alert from '$lib/components/ui/alert/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import CommunityModal from '../../../../../components/CommunityDialog.svelte';

	const { data }: PageProps = $props();
</script>

<div class="container">
	<div
		class="text-w-full mb-4 rounded-md border bg-muted px-4 py-2 text-lg font-semibold md:text-xl"
	>
		{data.community.name}
	</div>

	<div class="flex flex-col gap-4 py-0 lg:flex-row lg:items-start">
		<button
			class="relative cursor-pointer overflow-hidden rounded-lg border focus:outline-none"
			aria-label="Change community picture"
			type="button"
		>
			<img
				src="https://images.jdmagicbox.com/rep/b2b/wall-paper/wall-paper-11.jpg"
				alt="Solar panels on a field in summer"
				class="h-60 w-full object-cover ring-2 ring-transparent transition hover:ring-gray-300 lg:w-[30rem]"
			/>
			<div
				class="absolute inset-0 flex items-center justify-center rounded-lg bg-black/40 text-white opacity-0 transition hover:opacity-100"
			>
				Change
			</div>
		</button>

		<div class="grid flex-1 gap-4">
			<Alert.Root class="bg-blue-100">
				<Info />
				<Alert.Title>Success! Your changes have been saved</Alert.Title>
				<Alert.Description>This is an alert with icon, title and description.</Alert.Description>
			</Alert.Root>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="name" class="font-sm text-right">Name</Label>
				<Input
					id="name"
					value={data.community.name}
					class="col-span-3 text-sm"
					placeholder="Enter community name"
				/>
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="rule" class="font-sm text-right">Rule</Label>
				<Input id="rule" class="col-span-3 text-sm" placeholder="Enter community rule" />
			</div>
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="description" class="text-right">Description</Label>
				<Textarea
					id="description"
					class="col-span-3 h-0 overflow-y-auto text-sm"
					placeholder="Enter a short decription"
				/>
			</div>
		</div>
	</div>

	<div
		class="text-w-full my-4 rounded-md border bg-muted px-4 py-2 text-lg font-semibold md:text-xl"
	>
		Manage access
	</div>

	<Card.Root class="w-full ">
		<Card.Header>
			<div class="flex flex-col gap-2 pb-4 md:flex-row md:items-center">
				<div class="relative order-2 w-full md:order-none">
					<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
						<Search class="h-4 w-4 text-muted-foreground" />
					</div>
					<Input placeholder="Find a community..." class="flex-1 pl-9 text-sm md:text-base" />
				</div>
				<CommunityModal />
			</div>
		</Card.Header>
		<Card.Content></Card.Content>
	</Card.Root>

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
				class="w-full cursor-pointer rounded-md border border-red-600 bg-white px-4 py-2 text-red-600 hover:bg-red-50"
			>
				Leave Community
			</Button>
			<Button variant="destructive" class="w-full cursor-pointer rounded-md px-4 py-2">
				Delete Community
			</Button>
		</div>
	</div>
</div>
