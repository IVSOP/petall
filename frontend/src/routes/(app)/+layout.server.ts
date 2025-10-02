import type { User } from '$lib/api';
import type { MeResponse } from '$lib/api/auth';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ fetch, cookies }) => {
	const sessionId = cookies.get('sessionId') ?? '';
	const response = await fetch(`/api/auth/me`, {
		method: 'POST',
		headers: {
			Authorization: sessionId,
			'Content-Type': 'application/json'
		}
	});
	const me: User = await response.json();

	return {
		user: me,
		image: 'https://github.com/DigoqueDigo.png'
	};
};
