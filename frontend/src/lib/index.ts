export type User = {
	id: string;
	name: string;
	email: string;
};

export type UserCommunity = {
	user_id: string;
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

export type EnergyStats = {
	periodStart: Date;
	generatedSum: number;
	consumedSum: number;
	generatedPrice: number;
	consumedPrice: number;
};

export type ProcessedEnergy = {
	id: string;
	user: string;
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
