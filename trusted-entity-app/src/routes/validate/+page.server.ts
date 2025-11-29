import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getSession } from '$lib/sessions';

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
	const { proof, energyRecord } = queryData;
	return { proof, energyRecord };
};
