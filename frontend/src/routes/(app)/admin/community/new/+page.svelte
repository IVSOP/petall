<script lang="ts">
	import ErrorDialog from '$lib/components/ErrorDialog.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import type { PageProps } from './$types';

	const { form }: PageProps = $props();
</script>

<svelte:head>
	<title>New Community</title>
</svelte:head>

<div class="mx-auto max-w-2xl space-y-4">
	<!-- <h1 class="text-2xl font-semibold text-gray-900">Create Community</h1> -->

	<form method="POST" class="space-y-4">
		<div class="space-y-2">
			<Label for="name" class="text-sm font-medium">Name</Label>
			<Input
				id="name"
				name="name"
				type="text"
				placeholder="Community name"
				value={form?.name ?? ''}
				required
				class="h-10"
			/>
			<p class="text-xs text-muted-foreground">
				Enter a descriptive name for your community (eg. name of the company behind it)
			</p>
		</div>

		<div class="space-y-2">
			<Label for="description" class="text-sm font-medium">Description</Label>
			<Textarea
				id="description"
				name="description"
				placeholder="Add more information about the community"
				value={form?.description ?? ''}
				required
				rows={4}
				class="resize-none"
			/>
			<p class="text-xs text-muted-foreground">
				Describe your community energy details and locations
			</p>
		</div>

		<div class="space-y-2">
			<Label for="image" class="text-sm font-medium">Banner Image URL (Optional)</Label>
			<Input
				id="image"
				name="image"
				type="url"
				placeholder="https://example.com/image.jpg"
				value={form?.image ?? ''}
				class="h-10"
			/>
			<p class="text-xs text-muted-foreground">
				Optional: Add a banner image for better recognition
			</p>
		</div>

		<div class="pt-4">
			<Button type="submit" class="w-full cursor-pointer">Create Community</Button>
			<p class="mt-2 text-center text-xs text-muted-foreground">
				You will be assigned Manager of this community after creating it
			</p>
		</div>
	</form>

	{#if form?.error}
		<ErrorDialog>{form.error}</ErrorDialog>
	{/if}
</div>
