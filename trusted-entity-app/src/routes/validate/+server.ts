import { error, redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { validateJWT } from '$lib/jwt';
import { getOrCreateSession, SESSION_DURATION_MS } from '$lib/sessions';

const ZK_POC_SERVICE_URL =
	process.env.ZK_POC_SERVICE_URL || 'http://localhost:3002';

interface ValidateRequest {
	token: string;
}

interface ZKValidateResponse {
	proof: string;
	energy_record_cost: string;
}

export const POST: RequestHandler = async ({ request, cookies }) => {
	const body: ValidateRequest = await request.json();
	if (!body.token) {
		return error(400, 'Missing token in request body');
	}

	const claims = await validateJWT(body.token);
	const energyRecordId = claims.eri;
	const userId = claims.uid;

	const session = getOrCreateSession(userId);

	const zkResponse = await fetch(
		`${ZK_POC_SERVICE_URL}/validate/${energyRecordId}`,
		{
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			}
		}
	);

	if (!zkResponse.ok) {
		return error(500, 'Failed to query zk-poc-service');
	}

	const zkResult: ZKValidateResponse = await zkResponse.json();

	session.queries.set(energyRecordId, {
		proof: zkResult.proof,
		energy_record_cost: zkResult.energy_record_cost
	});
	cookies.set('session_id', session.id, {
		path: '/',
		httpOnly: true,
		sameSite: 'lax',
		secure: process.env.NODE_ENV === 'production',
		maxAge: SESSION_DURATION_MS / 1000 // Convert milliseconds to seconds
	});

	throw redirect(302, `/validate?query=${energyRecordId}`);
};

