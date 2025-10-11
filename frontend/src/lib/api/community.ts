import type { Community, EnergyRecord, UserRole } from '$lib';

export type CommunityCreateRequest = {
	name: string;
	description: string;
	image?: string;
};

export type CommunityCreateResponse = Community;

export type CommunityGetCommunitiesResponse = Community & { role: UserRole };
export type GetCommunitiesResponse = CommunityGetCommunitiesResponse[];

export type PaginatedEnergyRecords = {
	records: EnergyRecord[];
	totalCount: number;
};
