<script lang="ts">
	import type { PageProps } from './$types';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import '../../../app.css';

	let { form }: PageProps = $props();
</script>

<div class="flex min-h-svh flex-col items-center justify-center gap-6 bg-muted p-6 md:p-10">
	<div class="flex w-full max-w-sm flex-col gap-6">
		<div class="flex flex-col gap-6">
			<Card.Root>
				<Card.Header class="text-center">
					<Card.Title class="text-xl">Login</Card.Title>
					<Card.Description>Login with your account</Card.Description>
				</Card.Header>
				<Card.Content>
					{#if form?.error}
						<p
							class="mb-3 rounded-md border border-destructive/20 bg-destructive/10 p-3 text-sm text-destructive"
							role="alert"
						>
							{form.error}
						</p>
					{/if}
					<form method="POST">
						<div class="grid gap-6">
							<div class="grid gap-6">
								<div class="grid gap-3">
									<Label for="email">Email</Label>
									<Input id="email" name="email" type="text" value={form?.email ?? ''} required />
								</div>
								<div class="grid gap-3">
									<Label for="password">Password</Label>
									<Input id="password" name="password" type="password" required />
								</div>
								<Button type="submit" class="w-full">Sign in</Button>
							</div>
							<div class="text-center text-sm">
								Don't have an account?
								<a href="/register" class="underline underline-offset-4">Register</a>
							</div>
						</div>
					</form>
				</Card.Content>
			</Card.Root>
		</div>
	</div>
</div>
