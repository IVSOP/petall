import { v4 as randomUUID } from 'uuid';

export interface ValidationResult {
	proof: string;
	energyRecordCost: string; // BigDecimal as string
}

export interface Session {
	id: string;
	userId: string;
	expiresAt: Date;
	queries: Map<string, ValidationResult>; // Map of energy record ID -> validation result
}

const sessions = new Map<string, Session>();
export const SESSION_DURATION_MS = 24 * 60 * 60 * 1000;

export function getOrCreateSession(userId: string): Session {
	const now = new Date();
	for (const session of sessions.values()) {
		if (session.userId === userId && session.expiresAt > now) {
			session.expiresAt = new Date(now.getTime() + SESSION_DURATION_MS);
			return session;
		}
	}

	const session: Session = {
		id: randomUUID(),
		userId,
		expiresAt: new Date(now.getTime() + SESSION_DURATION_MS),
		queries: new Map()
	};

	sessions.set(session.id, session);
	return session;
}

export function getSession(sessionId: string): Session | undefined {
	const session = sessions.get(sessionId);
	if (!session) {
		return undefined;
	}

	if (session.expiresAt <= new Date()) {
		sessions.delete(sessionId);
		return undefined;
	}

	return session;
}
