<script lang="ts">
	import { Textarea } from '$lib/components/ui/textarea/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';

	const { community } = $props();
</script>

<div class="grid gap-4">
	<div class="text-w-full rounded-md border bg-muted px-4 py-2 text-lg font-semibold md:text-xl">
		{community.name}
	</div>
	<div class="flex flex-col gap-4 md:flex-row">
		<button
			class="relative cursor-pointer overflow-hidden rounded-lg border focus:outline-none"
			aria-label="Change community picture"
			type="button"
		>
			<img
				src="https://media.istockphoto.com/id/814423752/photo/eye-of-model-with-colorful-art-make-up-close-up.jpg?s=612x612&w=0&k=20&c=l15OdMWjgCKycMMShP8UK94ELVlEGvt7GmB_esHWPYE="
				alt="foo"
				class="h-60 w-full object-cover ring-2 ring-transparent transition hover:ring-gray-300 md:w-[20rem] lg:w-[30rem]"
			/>
			<div
				class="absolute inset-0 flex items-center justify-center rounded-lg bg-black/40 text-white opacity-0 transition hover:opacity-100"
			>
				Change
			</div>
		</button>
		<div class="grid flex-1 gap-4">
			<div class="grid gap-2">
				<Label for="name" class="font-sm text-right">Name</Label>
				<Input
					id="name"
					value={community.name}
					class="text-sm"
					placeholder="Enter community name"
				/>
			</div>
			<div class="grid gap-2">
				<Label for="rule" class="font-sm text-right">Rule</Label>
				<Input id="rule" class="text-sm" placeholder="Enter community rule" />
			</div>
			<div class="grid gap-2">
				<Label for="description" class="text-right">Description</Label>
				<Textarea id="description" class="h-0 text-sm" placeholder="Enter a short decription" />
			</div>
		</div>
	</div>
</div>
