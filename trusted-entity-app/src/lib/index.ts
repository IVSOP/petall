// place files you want to import through the `$lib` alias in this folder.

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
