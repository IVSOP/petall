<script lang="ts">
    import type { PageProps } from './$types';
    import * as Card from '$lib/components/ui/card/index.js';
    import { Button } from '$lib/components/ui/button/index.js';
    import { Label } from '$lib/components/ui/label/index.js';
    import { Input } from '$lib/components/ui/input/index.js';
    import '../../../app.css';
    import ErrorDialog from '$lib/components/ErrorDialog.svelte';

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
                        <ErrorDialog>
                            {form.error}
                        </ErrorDialog>
                    {/if}
                    <form method="POST">
                        <div class="grid gap-6">
                            <div class="grid gap-2">
                                <Button
                                    variant="outline"
                                    class="w-full"
                                    type="button"
                                    onclick={() => window.location.href = 'http://localhost:8080/auth/oauth/google'}
                                >
                                    <svg class="mr-2 h-4 w-4" viewBox="0 0 24 24">
                                        <path
                                            fill="currentColor"
                                            d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                                        />
                                        <path
                                            fill="currentColor"
                                            d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                                        />
                                        <path
                                            fill="currentColor"
                                            d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                                        />
                                        <path
                                            fill="currentColor"
                                            d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                                        />
                                    </svg>
                                    Continue with Google
                                </Button>
                            </div>
                            <div class="relative">
                                <div class="absolute inset-0 flex items-center">
                                    <span class="w-full border-t"></span>
                                </div>
                                <div class="relative flex justify-center text-xs uppercase">
                                    <span class="bg-background px-2 text-muted-foreground">Or continue with</span>
                                </div>
                            </div>

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
