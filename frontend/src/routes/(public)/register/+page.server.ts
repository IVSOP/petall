import { fail, redirect } from '@sveltejs/kit';
import type { RegisterRequest, RegisterResponse } from '$lib/api/auth';
import type { ErrorResponse } from '$lib/api';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, url }) => {
	const { user } = locals;

	if (user) {
		// Already logged in, redirect to home page
		throw redirect(302, '/');
	}

	const error = url.searchParams.get('error');

	return {
		error
	};
};

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

		const { sessionId }: RegisterResponse = await response.json();

		cookies.set('sessionId', sessionId, {
			path: '/',
			httpOnly: true
		});

		redirect(302, '/');

		return { success: true };
	}
} satisfies Actions;
