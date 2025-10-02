import type { User } from '$lib/api';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	// Skip user authentication for API routes to prevent infinite loop
	if (event.url.pathname.startsWith('/api/')) {
		return await resolve(event);
	}

	if (event.locals.user) {
		return await resolve(event);
	}

	const session = event.cookies.get('sessionId');
	if (!session) {
		return await resolve(event);
	}

	const response = await event.fetch('/api/auth/me', {
		method: 'POST',
		headers: {
			Authorization: session
		}
	});

	if (!response.ok) {
		return await resolve(event);
	}

	const user: User = await response.json();
	event.locals.user = user;

	return await resolve(event);
};
