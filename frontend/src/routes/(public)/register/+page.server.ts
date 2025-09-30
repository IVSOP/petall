import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import type { RegisterRequest, RegisterResponse } from '$lib/api/auth';
import type { ErrorResponse } from '$lib/api';

export const actions = {
	default: async ({ cookies, request, fetch }) => {
		const data = await request.formData();
		const name = data.get('name')?.toString().trim();
		const email = data.get('email')?.toString().trim();
		const password = data.get('password')?.toString();

		if (!name || !email || !password) {
			return fail(400, { name, email, error: 'Missing name, email or password' });
		}

		const registerRequest: RegisterRequest = {
			name,
			email,
			password
		};

		const response = await fetch('/api/auth/register', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(registerRequest)
		});

		if (!response.ok) {
			const error: ErrorResponse = await response.json();
			return fail(response.status, { name, email, error: error.error });
		}

		const { accessToken, refreshToken }: RegisterResponse = await response.json();

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
