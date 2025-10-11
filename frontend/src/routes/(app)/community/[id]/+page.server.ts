import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { EnergyRecord } from '$lib';
import type { PaginatedEnergyRecords } from '$lib/api/community';

export const load: PageServerLoad = async ({ fetch, params, cookies }) => {
	const sessionId = cookies.get('sessionId');
	const communityId = params.id;

	if (!sessionId) {
		redirect(302, '/login');
	}

	const response = await fetch(`/api/community/${params.id}/energy`, {
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

	if (response.status == 401) {
		redirect(302, '/login');
	}

	if (!response.ok) {
		throw new Error('Failed to fetch community');
	}

	const energyRecords: PaginatedEnergyRecords = await response.json();

	return {
		sessionId,
		communityId,
		energyRecords
	};
};
