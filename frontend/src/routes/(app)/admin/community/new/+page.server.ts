import { fail, redirect } from '@sveltejs/kit';
import type { PageServerLoad, Actions } from './$types';
import type { ErrorResponse } from '$lib/api';
import type { CommunityCreateRequest, CommunityCreateResponse } from '$lib/api/community';

export const load: PageServerLoad = async ({ cookies, locals }) => {
	const sessionId = cookies.get('sessionId');

	if (!sessionId) {
		throw redirect(302, '/login');
	}

	if (!locals.user || !locals.user.isAdmin) {
		throw redirect(302, '/');
	}
};

export const actions = {
	default: async ({ cookies, request, fetch }) => {
		const data = await request.formData();
		const name = data.get('name')?.toString().trim();
		const description = data.get('description')?.toString().trim();
		const image = data.get('image')?.toString();

		if (!name || !description) {
			return fail(400, { name, description, image, error: 'Missing name or description' });
		}

		const sessionId = cookies.get('sessionId');

		if (!sessionId) {
			return redirect(302, '/login');
		}

		const req: CommunityCreateRequest = {
			name,
			description,
			image
		};

		const response = await fetch('/api/admin/community', {
			method: 'POST',
			headers: {
				Authorization: sessionId,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(req)
		});

		if (response.status == 401) {
			return redirect(302, '/login');
		}

		if (!response.ok) {
			const error: ErrorResponse = await response.json();
			return fail(response.status, { name, description, image, error: error.error });
		}

		const community: CommunityCreateResponse = await response.json();
		redirect(302, `/community/${community.id}`);
	}
} satisfies Actions;
