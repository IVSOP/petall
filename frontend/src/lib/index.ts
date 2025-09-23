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
	coefficient: number;
	start: string;
	end: string;
};

export type ProcessedEnergy = {
    id: string;
	participant: string;
	community: string;
	generated: number;
	consumed: number;
	coefficient: number;
	start: Date;
	end: Date;
    delta: number,
}

export type Community = {
	id: string;
	entity: string;
	supplier: string;
};
