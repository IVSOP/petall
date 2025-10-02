import type { UserRoleData } from '$lib';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, locals }) => {
	const { user } = locals;

	if (!user) {
		throw redirect(302, '/login');
	}

	const response = await fetch(`/api/user/${user.id}/communities`);
	const communities: UserRoleData[] = await response.json();

	return {
		communities
	};
};
