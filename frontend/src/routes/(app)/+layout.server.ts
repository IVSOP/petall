import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import type { GetCommunitiesResponse as CommunityResponse } from '$lib/api/community';


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

	const communities: CommunityResponse = await response.json();
	const pathParts = url.pathname.split('/');

	console.log(pathParts)

	if (communities.length >= 1 && pathParts.length == 2 && pathParts[1] === '') {
		throw redirect(
			307,
			`/community/${communities[0].id}`
		);
	}

	return {
		communities,
		user: locals.user
	};
};
