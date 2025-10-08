import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import type { GetCommunitiesResponse as CommunityResponse } from '$lib/api/community';


export const load: LayoutServerLoad = async ({ fetch, locals, cookies }) => {
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

	const communities: CommunityResponse = await response.json();

	return {
		communities,
		user: locals.user
	};
};
