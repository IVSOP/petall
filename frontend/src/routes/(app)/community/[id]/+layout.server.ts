import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import type { Community } from '$lib';

export const load: LayoutServerLoad = async ({ fetch, params, cookies }) => {
	const sessionId = cookies.get('sessionId');

	if (!sessionId) {
		redirect(302, '/login');
	}

	const communityResponse = await fetch(`/api/community/${params.id}`, {
		method: 'GET',
		headers: {
			Authorization: sessionId,
			'Content-Type': 'application/json'
		}
	});

	if (communityResponse.status == 401) {
		redirect(302, '/login');
	}

	if (!communityResponse.ok) {
		throw new Error('Failed to fetch community');
	}

	const community: Community = await communityResponse.json();

	console.log(community);

	return {
		community
	};
};
