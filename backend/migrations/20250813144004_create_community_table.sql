CREATE TABLE IF NOT EXISTS "community" (
	"id" UUID NOT NULL UNIQUE,
	"entity" VARCHAR(255) NOT NULL UNIQUE,
	PRIMARY KEY("id")
);

CREATE INDEX IF NOT EXISTS community_entity_idx ON community (entity);
