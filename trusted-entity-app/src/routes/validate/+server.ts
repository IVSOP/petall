import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { validateJWT } from '$lib/jwt';
import { getOrCreateSession, SESSION_DURATION_MS } from '$lib/sessions';
import type { EnergyRecord } from '$lib';

const ZK_POC_SERVICE_URL = process.env.ZK_POC_SERVICE_URL || 'http://localhost:3002';

const ALLOWED_ORIGIN = process.env.ALLOWED_ORIGIN || 'http://localhost:5173';

interface ValidateRequest {
	token: string;
}

interface ZKValidateResponse {
	proof: string;
	energyRecord: EnergyRecord;
}

function setCorsHeaders(response: Response, origin: string | null): void {
	if (!origin || origin !== ALLOWED_ORIGIN) {
		return;
	}

	response.headers.set('Access-Control-Allow-Origin', origin);
	response.headers.set('Access-Control-Allow-Methods', 'POST, OPTIONS');
	response.headers.set('Access-Control-Allow-Headers', 'Content-Type');
	response.headers.set('Access-Control-Allow-Credentials', 'true');
}

export const OPTIONS: RequestHandler = async ({ request }) => {
	const origin = request.headers.get('origin');
	const response = new Response(null, { status: 204 });
	setCorsHeaders(response, origin);
	return response;
};

export const POST: RequestHandler = async ({ request, cookies }) => {
	const origin = request.headers.get('origin');
	const body: ValidateRequest = await request.json();
	if (!body.token) {
		const errorResponse = json({ error: 'Missing token in request body' }, { status: 400 });
		setCorsHeaders(errorResponse, origin);
		return errorResponse;
	}

	const claims = await validateJWT(body.token);
	const energyRecordId = claims.eri;
	const userId = claims.uid;

	const session = getOrCreateSession(userId);

	const zkResponse = await fetch(`${ZK_POC_SERVICE_URL}/validate/${energyRecordId}`, {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});

	if (!zkResponse.ok) {
		const errorResponse = json({ error: 'Failed to query zk-poc-service' }, { status: 500 });
		setCorsHeaders(errorResponse, origin);
		return errorResponse;
	}

	const zkResult: ZKValidateResponse = await zkResponse.json();

	session.queries.set(energyRecordId, {
		proof: zkResult.proof,
		energyRecord: zkResult.energyRecord
	});

	// Not sure if this works with different domains but whatever...
	cookies.set('trustedEntitySessionId', session.id, {
		path: '/',
		maxAge: SESSION_DURATION_MS / 1000 // Convert milliseconds to seconds
	});

	const successResponse = json({ query: energyRecordId }, { status: 200 });
	setCorsHeaders(successResponse, origin);
	return successResponse;
};
