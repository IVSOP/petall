import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import type { LoginRequest, LoginResponse } from '$lib/api/auth';
import type { ErrorResponse } from '$lib/api';

export const load: PageServerLoad = async ({ locals }) => {
	const { user } = locals;

	if (user) {
		// Already logged in, redirect to home page
		throw redirect(302, '/');
	}
};

export const actions = {
	default: async ({ cookies, request, fetch, url }) => {
		const data = await request.formData();
		const email = data.get('email')?.toString().trim();
		const password = data.get('password')?.toString();

		if (!email || !password) {
			return fail(400, { email, error: 'Missing email or password' });
		}

		const loginRequest: LoginRequest = {
			email,
			password
		};

		const response = await fetch('/api/auth/login', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(loginRequest)
		});

		if (!response.ok) {
			const error: ErrorResponse = await response.json();
			return fail(response.status, { email, error: error.error });
		}

		const { sessionId }: LoginResponse = await response.json();

		cookies.set('sessionId', sessionId, {
			path: '/',
			httpOnly: true
		});

		if (url.searchParams.has('redirectTo')) {
			redirect(303, url.searchParams.get('redirectTo') ?? '/');
		} else {
			redirect(303, '/');
		}

		return { success: true };
	}
} satisfies Actions;
