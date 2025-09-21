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

export type Energy = {
	id: string;
	participant: string;
	community: string;
	generated: number;
	consumed: number;
    coefficient: number,
    start: string,
	end: string;
    delta?: number, // is null at the start
};

export type Community = {
	id: string;
	entity: string;
	supplier: string;
};
