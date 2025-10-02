import type { Community, UserRole } from '$lib';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { GetCommunitiesResponse as CommunityResponse } from '$lib/api/community';

export const load: PageServerLoad = async ({ fetch, cookies }) => {
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
		communities
	};
};
