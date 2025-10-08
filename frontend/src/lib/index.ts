export type Participant = {
	id: string;
	name: string;
	email: string;
};

export type ParticipantCommunity = {
	participant_id: string;
	community_id: string;
};

export type EnergyRecord = {
	id: string;
	userId: string;
	communityId: string;
	generated: number;
	consumed: number;
	consumerPrice: number;
	sellerPrice: number;
	start: string;
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
	delta: number;
};

export type Community = {
	id: string;
	name: string;
	supplier: string;
	description: string;
};

export type UserRole = 'Manager' | 'User';
