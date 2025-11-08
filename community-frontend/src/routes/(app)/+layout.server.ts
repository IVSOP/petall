import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import type { GetCommunitiesResponse } from '$lib/api/community';

export const load: LayoutServerLoad = async ({ fetch, locals, cookies, url }) => {
	const sessionId = cookies.get('sessionId');

	if (!sessionId) {
		throw redirect(302, '/login');
	}

	const response = await fetch(`/api/community`, {
		method: 'GET',
		headers: {
			Authorization: sessionId
		}
	});

	if (response.status == 401) {
		throw redirect(302, '/login');
	}

	const communities: GetCommunitiesResponse = await response.json();

	// Redirect to first community if at root path
	if (communities.length > 0 && url.pathname === '/') {
		throw redirect(302, `/community/${communities[0].id}`);
	}

	return {
		communities,
		user: locals.user
	};
};
