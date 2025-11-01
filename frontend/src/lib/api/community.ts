import type { Community, EnergyRecord } from '$lib';

export type CommunityCreateRequest = {
	name: string;
	description: string;
	image?: string;
};

export type CommunityCreateResponse = Community;

export type GetCommunitiesResponse = (Community & { isPresent: boolean })[];

export type PaginatedEnergyRecords = {
	records: EnergyRecord[];
	totalCount: number;
};
