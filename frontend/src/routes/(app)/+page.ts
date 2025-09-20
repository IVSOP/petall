import type { Community } from '$lib';
import type { PageLoad } from './$types';

// TODO: get this from login
const participantId = '80ad3f62-853b-4370-b345-8b49ea1cef9b';

export const load: PageLoad = async ({ fetch }) => {
	const response = await fetch(`/api/participant/${participantId}/communities`);
	const communities: Community[] = await response.json();

	return {
		communities
	};
};
