import type { Participant } from '$lib';
import type { LayoutLoad } from './$types';

// TODO: get this from login
const participantId = '80ad3f62-853b-4370-b345-8b49ea1cef9b';

export const load: LayoutLoad = async ({ fetch }) => {
	const response = await fetch(`/api/participant/${participantId}`);
	const participant: Participant = await response.json();

	return {
		participant,
		"image": "https://github.com/DigoqueDigo.png"
	};
};