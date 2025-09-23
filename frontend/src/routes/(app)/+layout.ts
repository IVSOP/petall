import type { Participant } from '$lib';
import type { LayoutLoad } from './$types';

// TODO: get this from login
const participantId = '63e3c963-4a91-486A-bffe-9deed57864b2';

export const load: LayoutLoad = async ({ fetch }) => {
	const response = await fetch(`/api/participant/${participantId}`);
	const participant: Participant = await response.json();

	return {
		participant,
		image: 'https://github.com/DigoqueDigo.png'
	};
};
