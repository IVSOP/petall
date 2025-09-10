import type { Community, EnergyTransfer } from '$lib';
import type { PageLoad } from './$types';

// TODO: get this from login
const participantId = '062fe509-57e0-4b7a-9eb1-afc7dbd0685a';

export const load: PageLoad = async ({ fetch, params }) => {
	// TODO: merge this into one request and backend does all the handling
	const community: Community = await fetch(`/api/community/${params.id}`).then((res) => res.json());

	const energyTransfers: EnergyTransfer[] = await fetch(
		`/api/community/${params.id}/energytransfer/${participantId}?page=1`
	).then((res) => res.json());

	return {
		community,
		energyTransfers
	};
};
