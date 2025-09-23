import type { Community } from '$lib';
import type { PageLoad } from './$types';

// TODO: get this from login
const participantId = '63e3c963-4a91-486A-bffe-9deed57864b2';

export const load: PageLoad = async ({ fetch }) => {
	const response = await fetch(`/api/participant/${participantId}/communities`);
	const communities: Community[] = await response.json();

	return {
		communities
	};
};
