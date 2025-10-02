import type { MeResponse } from '$lib/api/auth';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ fetch, cookies }) => {
	const accessToken = cookies.get('accessToken') ?? '';
	const response = await fetch(`/api/auth/me`, {
		method: 'POST',
		headers: {
			Authorization: accessToken,
			'Content-Type': 'application/json'
		}
	});
	const me: MeResponse = await response.json();

	return {
		...me,
		image: 'https://github.com/DigoqueDigo.png'
	};
};
