import type { Participant } from '$lib';
import type { LayoutLoad } from './$types';

// TODO: get this from login
const participantId = '50a536d5-755b-4904-af39-b9e30c6b8a58';

export const load: LayoutLoad = async ({ fetch }) => {
	const response = await fetch(`/api/participant/${participantId}`);
	const participant: Participant = await response.json();

	return {
		participant,
		image: 'https://github.com/DigoqueDigo.png'
	};
};
