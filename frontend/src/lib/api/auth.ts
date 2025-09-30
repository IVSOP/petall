export type RegisterRequest = { name: string; email: string; password: string };
export type RegisterResponse = {
	uuid: string;
	name: string;
	email: string;
	accessToken: string;
	refreshToken: string;
};

export type LoginRequest = { email: string; password: string };
export type LoginResponse = { accessToken: string; refreshToken: string };

export type RefreshTokenRequest = { refreshToken: string };
export type RefreshTokenResponse = { accessToken: string };

export type ChangePasswordRequest = { oldPassword: string; newPassword: string };
export type ChangePasswordResponse = { message: string; accessToken: string; refreshToken: string };

export type RevokeResponse = { message: string };
export type MeResponse = { id: string; email: string; name: string };
