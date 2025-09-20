export type Participant = {
	id: string;
	role: 'manager' | 'supplier' | 'user';
	name: string;
	email: string;
};

export type ParticipantCommunity = {
	participant_id: string;
	community_id: string;
};

export type EnergyTransfer = {
	id: string;
	participant_from: string;
	participant_to: string;
	community: string;
	energy_wh: number;
	start: string;
	end: string;
};

export type Community = {
	id: string;
	entity: string;
	supplier: string;
};
