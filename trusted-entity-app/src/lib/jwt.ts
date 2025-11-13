import { jwtVerify, importSPKI } from 'jose';
import { readFileSync } from 'fs';
import { join } from 'path';

export interface ValidationClaims {
	uid: string; // User ID
	eri: string; // Energy Record ID
	exp: number; // Expiration timestamp
}

function loadPublicKeyPEM(): string {
	const publicKeyPath =
		process.env.VALIDATION_PUBLIC_KEY_PATH || join(process.cwd(), '..', 'keys', 'validation.key.pub');
	return readFileSync(publicKeyPath, 'utf-8');
}

const publicKey = await importSPKI(loadPublicKeyPEM(), 'RS256');

export async function validateJWT(token: string): Promise<ValidationClaims> {
	const { payload } = await jwtVerify(token, publicKey, {
		algorithms: ['RS256']
	});

	return {
		uid: payload.uid as string,
		eri: payload.eri as string,
		exp: payload.exp as number
	};
}

