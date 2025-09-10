import type { Community, EnergyTransfer } from '$lib';
import type { PageLoad } from './$types';

// TODO: get this from login
const participantId = '80ad3f62-853b-4370-b345-8b49ea1cef9b';

export const load: PageLoad = async ({ fetch, params }) => {
	// TODO: merge this into one request and backend does all the handling
	const community: Community = await fetch(`/api/community/${params.id}`).then((res) => res.json());

	const energyTransfers: EnergyTransfer[] = await fetch(
		`/api/community/${params.id}/energytransfer/${participantId}?page=1&size=20`
	).then((res) => res.json());

	energyTransfers.forEach((item) => {
		item.energy_wh = Number(item.energy_wh);
		if (participantId == item.participant_to) {
			item.energy_wh = -item.energy_wh;
		}
	});

	return {
		community,
		energyTransfers,
		participantId
	};
};
