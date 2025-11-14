import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getSession } from '$lib/sessions';
import type { EnergyRecord } from '$lib';
import { env } from '$env/dynamic/public';

const COMMUNITY_BACKEND_URL = env.PUBLIC_COMMUNITY_BACKEND_URL || 'http://localhost:8080';

export const load: PageServerLoad = async ({ cookies, url }) => {
	const sessionId = cookies.get('trustedEntitySessionId');
	const query = url.searchParams.get('query');

	if (!sessionId) {
		error(401, 'Missing session ID in cookie');
	}

	if (!query) {
		error(400, 'Missing query parameter');
	}

	const session = getSession(sessionId);
	if (!session) {
		error(401, 'Invalid or expired session');
	}

	const queryData = session.queries.get(query);
	if (!queryData) {
		error(404, 'Query not found in session');
	}

	const energyRecordResponse = await fetch(
		`${COMMUNITY_BACKEND_URL}/energy-record-unauthenticated/${query}`,
		{
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		}
	);

	if (!energyRecordResponse.ok) {
		error(404, 'Can not fetch energy record');
	}

	const energyRecordData: EnergyRecord = await energyRecordResponse.json();

	return {
		proof: queryData.proof,
		energyRecordCost: queryData.energyRecordCost,
		energyRecordData: energyRecordData
	};
};
