import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { OAuthCallbackResponse } from '$lib/api/auth';
import type { ErrorResponse } from '$lib/api';

export const load: PageServerLoad = async ({ url, cookies, fetch }) => {
	const code = url.searchParams.get('code');
	const error = url.searchParams.get('error');

	if (error) {
		throw redirect(303, `/login?error=${encodeURIComponent(error)}`);
	}

	if (!code) {
		throw redirect(303, '/login?error=missing_code');
	}

	const response = await fetch(`/api/auth/callback?code=${encodeURIComponent(code)}`);

	if (!response.ok) {
		const errorData: ErrorResponse = await response.json();
		throw redirect(303, `/login?error=${encodeURIComponent(errorData.error)}`);
	}

	const data: OAuthCallbackResponse = await response.json();

	cookies.set('sessionId', data.sessionId, {
		path: '/',
		httpOnly: true
	});

	if (data.isNewUser) {
		throw redirect(303, '/'); // TODO: add welcome page?
	} else {
		throw redirect(303, '/');
	}
};
