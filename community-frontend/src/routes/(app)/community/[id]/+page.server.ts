import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { Community } from '$lib';
import type { PaginatedEnergyRecords } from '$lib/api/community';

export const load: PageServerLoad = async ({ fetch, params, cookies }) => {
	const sessionId = cookies.get('sessionId');

	if (!sessionId) {
		redirect(302, '/login');
	}

	const energyResponse = await fetch(`/api/community/${params.id}/energy`, {
		method: 'POST',
		headers: {
			Authorization: sessionId,
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			page: 1,
			size: 10,
			orderDir: 'desc'
		})
	});

	const communityResponse = await fetch(`/api/community/${params.id}`, {
		method: 'GET',
		headers: {
			Authorization: sessionId,
			'Content-Type': 'application/json'
		}
	});

	if (energyResponse.status == 401 || communityResponse.status == 401) {
		redirect(302, '/login');
	}

	if (!energyResponse.ok || !communityResponse.ok) {
		throw new Error('Failed to fetch community');
	}

	const energyRecords: PaginatedEnergyRecords = await energyResponse.json();
	const community: Community = await communityResponse.json();

	return {
		sessionId,
		community,
		energyRecords
	};
};
