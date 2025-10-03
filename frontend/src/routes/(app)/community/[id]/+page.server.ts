import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { CommunityGetCommunitiesResponse } from '$lib/api/community';

export const load: PageServerLoad = async ({ fetch, params, cookies }) => {
	const sessionId = cookies.get('sessionId');
	if (!sessionId) {
		redirect(302, '/login');
	}

	const response = await fetch(`/api/community/${params.id}`, {
		headers: {
			Authorization: sessionId
		}
	});

	if (response.status == 401) {
		redirect(302, '/login');
	}

	if (!response.ok) {
		throw new Error('Failed to fetch community');
	}

	const community: CommunityGetCommunitiesResponse = await response.json();

	return {
		community
	};
};
