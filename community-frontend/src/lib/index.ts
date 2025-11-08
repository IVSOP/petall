export type User = {
	id: string;
	name: string;
	email: string;
};

export type UserCommunity = {
	user_id: string;
	community_id: string;
};

export type ManagerCommunity = {
	manager_id: string;
	community_id: string;
};

export type Community = {
	id: string;
	name: string;
	image?: string;
	description: string;
	// rule: string;
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
