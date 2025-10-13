import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url, cookies }) => {
	const sessionId = url.searchParams.get('sessionId');
	const isNewUser = url.searchParams.get('isNewUser') === 'true';
	const error = url.searchParams.get('error');

	if (error) {
		throw redirect(303, `/login?error=${encodeURIComponent(error)}`);
	}

	if (!sessionId) {
		throw redirect(303, '/login?error=missing_session');
	}

	cookies.set('sessionId', sessionId, {
		path: '/',
		httpOnly: true,
	});

	if (isNewUser) {
		throw redirect(303, '/');
	} else {
		throw redirect(303, '/');
	}
};
