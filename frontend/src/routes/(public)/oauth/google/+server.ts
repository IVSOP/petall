import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import type { OAuthUrlResponse } from '$lib/api/auth';

export const GET: RequestHandler = async ({ fetch }) => {
	const response = await fetch('/api/auth/oauth/google');
	
	if (!response.ok) {
		const errorText = await response.text();
		console.error('OAuth init failed:', response.status, errorText);
		throw redirect(303, '/login?error=oauth_init_failed');
	}
	
	const data: OAuthUrlResponse = await response.json();
	throw redirect(303, data.authorizationUrl);
};
