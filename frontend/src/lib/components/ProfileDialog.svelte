<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Skeleton } from '$lib/components/ui/skeleton/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

	let isLoading = true;
	let image: string | null = 'https://github.com/DigoqueDigo.png';

	function handleFileChange(e: Event) {
		const target = e.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			const reader = new FileReader();
			reader.onload = (event) => {
				image = event.target?.result as string;
			};
			reader.readAsDataURL(file);
		}
	}

	function triggerFileInput() {
		document.getElementById('avatar-upload')?.click();
	}
</script>

<Dialog.Root>
	<Dialog.Trigger class="w-full focus:outline-none">
		<DropdownMenu.Item onSelect={(e) => e.preventDefault()} class="cursor-pointer">
			Edit Profile
		</DropdownMenu.Item>
	</Dialog.Trigger>

	<Dialog.Content class="sm:max-w-[500px]">
		<Dialog.Header>
			<Dialog.Title>Edit profile</Dialog.Title>
			<Dialog.Description>
				Make changes to your profile here. Click save when you're done.
			</Dialog.Description>
		</Dialog.Header>

		<div class="flex flex-col gap-6 py-0 sm:flex-row sm:items-start">
			<div class="flex items-center justify-center">
				<button
					class="relative h-28 w-28 flex-shrink-0 cursor-pointer rounded-full focus:outline-none sm:h-35 sm:w-35"
					on:click={triggerFileInput}
					aria-label="Change profile picture"
					type="button"
				>
					{#if isLoading}
						<Skeleton class="h-full w-full rounded-full" />
					{/if}

					<img
						src={image}
						alt=""
						class="h-full w-full rounded-full object-cover ring-2 ring-transparent transition hover:ring-gray-300 {isLoading
							? 'hidden'
							: ''}"
						on:load={() => (isLoading = false)}
					/>
					<div
						class="absolute inset-0 flex items-center justify-center rounded-full bg-black/40 text-white opacity-0 transition hover:opacity-100"
					>
						Change
					</div>
				</button>
			</div>

			<div class="grid flex-1 gap-4">
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="name" class="font-sm text-right">Name</Label>
					<Input
						id="name"
						value="Diogo Marques"
						class="col-span-3 text-sm"
						placeholder="Enter your name"
					/>
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="password" class="font-sm text-right">Password</Label>
					<Input
						id="password"
						type="password"
						value="password"
						class="col-span-3 text-sm"
						placeholder="Enter your password"
					/>
				</div>
				<div class="grid grid-cols-4 items-center gap-4">
					<Label for="confirm-password" class="font-sm text-right ">Confirm</Label>
					<Input
						id="confirm-password"
						type="password"
						value="password"
						class="col-span-3 text-sm"
						placeholder="Confirm your password"
					/>
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Button type="submit">Save changes</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<input
	id="avatar-upload"
	type="file"
	accept="image/*"
	class="hidden"
	on:change={handleFileChange}
/>
