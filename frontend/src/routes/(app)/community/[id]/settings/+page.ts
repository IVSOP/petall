import type { Community } from '$lib';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, params }) => {
	const community: Community = await fetch(`/api/community/${params.id}`).then((res) => res.json());
	return {
		community
	};
};
