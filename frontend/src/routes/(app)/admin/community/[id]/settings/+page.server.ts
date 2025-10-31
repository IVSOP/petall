import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import type { Community, User } from '$lib';
import type { ErrorResponse } from '$lib/api';

export const load: PageServerLoad = async ({ fetch, params, cookies }) => {
	const sessionId = cookies.get('sessionId');

	if (!sessionId) {
		throw redirect(302, '/login');
	}

	const response = await fetch(`/api/admin/community/${params.id}`, {
		method: 'GET',
		headers: {
			Authorization: sessionId
		}
	});

	if (!response.ok) {
		throw new Error('Failed to fetch community');
	}

	const admin_community_info: { community: Community } & { users: User[] } & {
		managers: User[];
	} & { admins: User[] } = await response.json();

	return {
		sessionId,
		admin_community_info
	};
};

export const actions: Actions = {
	addUser: async ({ cookies, params, request, fetch }) => {
		const data = await request.formData();
		const user_type = data.get('user_type')?.toString();
		const user_email = data.get('user_email')?.toString().trim();

		const sessionId = cookies.get('sessionId');

		if (!sessionId) {
			throw redirect(302, '/login');
		}

		if (!user_email) {
			return fail(400, { user_email, error: 'Missing email' });
		}

		const response = await fetch(`/api/admin/community/${params.id}/${user_type}`, {
			method: 'PUT',
			headers: {
				Authorization: sessionId,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				user_email: user_email
			})
		});

		if (!response.ok) {
			const error: ErrorResponse = await response.json();
			console.log(error);
			return fail(response.status, { user_email, error: error.error });
		}

		return { success: true };
	},

	removeUser: async ({ request, params, cookies, fetch }) => {
		const data = await request.formData();
		const user_type = data.get('user_type')?.toString();
		const user_email = data.get('user_email')?.toString();

		const sessionId = cookies.get('sessionId');
		if (!sessionId) {
			throw redirect(302, '/login');
		}

		if (!user_email) {
			return fail(400, { error: 'Missing email' });
		}

		const response = await fetch(`/api/admin/community/${params.id}/${user_type}`, {
			method: 'DELETE',
			headers: {
				Authorization: sessionId,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				user_email: user_email
			})
		});

		if (!response.ok) {
			const error: ErrorResponse = await response.json();
			console.log(error);
			return fail(response.status, { user_email, error: error.error });
		}

		console.log('aqui');

		return { success: true };
	}
} satisfies Actions;
