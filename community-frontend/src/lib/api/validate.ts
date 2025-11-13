import type { EnergyRecord } from '$lib';
import { env } from '$env/dynamic/public';

const TRUSTED_ENTITY_APP_URL = env.PUBLIC_TRUSTED_ENTITY_APP_URL || 'http://localhost:3010';

export async function handleValidate(record: EnergyRecord, session_id: string) {
	// Get the JWT token
	const response = await fetch(`/api/sign-energy-record-validation/${record.id}`, {
		method: 'GET',
		headers: {
			Authorization: session_id,
			'Content-Type': 'application/json'
		}
	});

	if (!response.ok) {
		console.error('Validation failed');
		return;
	}

	const tokenData: { signedRequest: string } = await response.json();

	const validateResponse = await fetch(`${TRUSTED_ENTITY_APP_URL}/validate`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ token: tokenData.signedRequest }),
		credentials: 'include' // allow for the fetch to set the cookies in the trusted entity app
	});

	if (!validateResponse.ok) {
		const errorText = await validateResponse.text();
		console.error('Validation request failed:', validateResponse.status, errorText);
		return;
	}

	const result: { query: string } = await validateResponse.json();
	const redirectUrl = `${TRUSTED_ENTITY_APP_URL}/validate?query=${result.query}`;
	window.open(redirectUrl, '_blank');
}
