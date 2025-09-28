import type { Community, Energy, ProcessedEnergy } from '$lib';
import type { PageLoad } from './$types';

// TODO: get this from login
const participantId = '50a536d5-755b-4904-af39-b9e30c6b8a58';

export const load: PageLoad = async ({ fetch, params }) => {
	// TODO: merge this into one request and backend does all the handling
	const community: Community = await fetch(`/api/community/${params.id}`).then((res) => res.json());

	const rawEnergyTransfers: Energy[] = await fetch(
		`/api/community/${params.id}/energy/${participantId}?size=10&orderDir=desc&page=1`
	).then((res) => res.json());

	const energyTransfers: ProcessedEnergy[] = rawEnergyTransfers.map((item) => {
		return {
			id: item.id,
			participant: item.participant,
			community: item.community,
			generated: item.generated,
			consumed: item.consumed,
			coefficient: item.coefficient,
			start: new Date(item.start),
			end: new Date(item.end),
			delta: item.generated - item.consumed
		};
	});

	return {
		community,
		energyTransfers,
		participantId
	};
};
