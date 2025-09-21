import type { Community, Energy } from '$lib';
import type { PageLoad } from './$types';

// TODO: get this from login
const participantId = '835d0b4e-98c3-447d-9cd8-db03a7f76e8e';

export const load: PageLoad = async ({ fetch, params }) => {
	// TODO: merge this into one request and backend does all the handling
	const community: Community = await fetch(`/api/community/${params.id}`).then((res) => res.json());

	const energyTransfers: Energy[] = await fetch(
		`/api/community/${params.id}/energy/${participantId}?size=10&orderDir=desc&page=1`
	).then((res) => res.json());

    energyTransfers.forEach((item) => {
        item.delta = item.generated - item.consumed;
    });

	return {
		community,
		energyTransfers,
		participantId
	};
};
