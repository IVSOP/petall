import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import type { LoginRequest, LoginResponse } from '$lib/api/auth';
import type { ErrorResponse } from '$lib/api';

export const actions = {
	default: async ({ cookies, request, fetch }) => {
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

		const { accessToken, refreshToken }: LoginResponse = await response.json();

		cookies.set('accessToken', accessToken, {
			path: '/',
			httpOnly: true
		});
		cookies.set('refreshToken', refreshToken, {
			path: '/',
			httpOnly: true
		});

		redirect(302, '/');

		return { success: true };
	}
} satisfies Actions;
