export type RegisterRequest = { name: string; email: string; password: string };
export type RegisterResponse = {
	uuid: string;
	name: string;
	email: string;
	sessionId: string;
};

export type LoginRequest = { email: string; password: string };
export type LoginResponse = {
	uuid: string;
	name: string;
	email: string;
	sessionId: string;
};

export type ChangePasswordRequest = { oldPassword: string; newPassword: string };
export type ChangePasswordResponse = { message: string; sessionId: string };

export type RevokeResponse = { message: string };
export type MeResponse = { id: string; email: string; name: string };

export type OAuthCallbackResponse = {
	uuid: string;
	name: string;
	email: string;
	sessionId: string;
	isNewUser: boolean;
};
