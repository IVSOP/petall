import type { UserRoleData } from '$lib';
import type { PageLoad } from './$types';

// TODO: get this from login
const participantId = '50a536d5-755b-4904-af39-b9e30c6b8a58';

export const load: PageLoad = async ({ fetch }) => {
	const response = await fetch(`/api/participant/${participantId}/communities`);
	const communities: UserRoleData[] = await response.json();

	return {
		communities
	};
};
