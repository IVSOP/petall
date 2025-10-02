import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ cookies, locals, fetch }) => {
	const session = cookies.get('sessionId');

	if (session) {
		cookies.delete('sessionId', { path: '/' });

		await fetch('/api/auth/revoke', {
			method: 'POST',
			headers: {
				Authorization: session
			}
		});
	}

	locals.user = null;

	throw redirect(302, '/login');
};
