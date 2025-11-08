import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { Community } from '$lib';

export const load: PageServerLoad = async ({ fetch, cookies, locals }) => {
	const sessionId = cookies.get('sessionId');

	if (!sessionId) {
		throw redirect(302, '/login');
	}

	if (!locals.user || !locals.user.canAccessAdminView) {
		throw redirect(302, '/');
	}

	const response = await fetch(`/api/admin/community`, {
		method: 'GET',
		headers: {
			Authorization: sessionId
		}
	});

	const communities: (Community & { userCount: string })[] = await response.json();

	return {
		communities
	};
};
