import type { Participant } from '$lib';
import type { LayoutLoad } from './$types';

// TODO: get this from login
const participantId = '98b74f06-a63c-4403-ba7b-e77e50143de3';

export const load: LayoutLoad = async ({ fetch }) => {
	const response = await fetch(`/api/participant/${participantId}`);
	const participant: Participant = await response.json();

	return {
		participant,
		image: 'https://github.com/DigoqueDigo.png'
	};
};
