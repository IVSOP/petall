import type { Community } from '$lib';
import type { PageLoad } from './$types';

// TODO: get this from login
const participantId = '98b74f06-a63c-4403-ba7b-e77e50143de3';

export const load: PageLoad = async ({ fetch }) => {
	const response = await fetch(`/api/participant/${participantId}/communities`);
	const communities: Community[] = await response.json();

	return {
		communities
	};
};
