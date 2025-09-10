import type { Community } from '$lib';
import type { PageLoad } from './$types';

// TODO: get this from login
const participantId = '062fe509-57e0-4b7a-9eb1-afc7dbd0685a';

export const load: PageLoad = async ({ fetch }) => {
	const response = await fetch(`/api/participant/${participantId}/communities`);
	const communities: Community[] = await response.json();

	return {
		communities
	};
};
