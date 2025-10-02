import type { Community, UserRole } from '$lib';

export type CommunityCreateRequest = {
	name: string;
	description: string;
	image?: string;
};

export type CommunityCreateResponse = Community;

export type CommunityGetCommunitiesResponse = Community & { role: UserRole };
export type GetCommunitiesResponse = CommunityGetCommunitiesResponse[];
