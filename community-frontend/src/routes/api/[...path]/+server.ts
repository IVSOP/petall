import { env } from '$env/dynamic/private';
import type { RequestHandler } from './$types';

const BACKEND_URL = env.BACKEND_URL || 'http://localhost:8080';

async function proxyRequest(method: string, path: string, request: Request) {
	const originalUrl = new URL(request.url);
	const url = `${BACKEND_URL}/${path}${originalUrl.search}`;

	const options: RequestInit = {
		method,
		headers: request.headers
	};

	if (method !== 'GET' && method !== 'HEAD') {
		options.body = await request.text();
	}

	return await fetch(url, options);
}

export const GET: RequestHandler = async ({ params, request }) => {
	return await proxyRequest('GET', params.path, request);
};

export const POST: RequestHandler = async ({ params, request }) => {
	return await proxyRequest('POST', params.path, request);
};

export const PUT: RequestHandler = async ({ params, request }) => {
	return await proxyRequest('PUT', params.path, request);
};

export const DELETE: RequestHandler = async ({ params, request }) => {
	return await proxyRequest('DELETE', params.path, request);
};
