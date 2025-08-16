CREATE TYPE participant_role AS ENUM (
    'manager',
    'supplier',
    'user'
);

CREATE TABLE IF NOT EXISTS "participant" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "role" participant_role NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL UNIQUE
);

CREATE INDEX IF NOT EXISTS "participant_email_idx" ON "participant" ("email");

CREATE TABLE IF NOT EXISTS "community" (
	"id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	"entity" VARCHAR(255) NOT NULL UNIQUE,
	"supplier" UUID NOT NULL,
	CONSTRAINT fk_supplier
        FOREIGN KEY ("supplier") 
        REFERENCES "participant"("id")
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "community_entity_idx" ON "community" ("entity");

CREATE TABLE IF NOT EXISTS "participant_community" (
    "participant_id" UUID NOT NULL,
    "community_id" UUID NOT NULL,
    PRIMARY KEY ("participant_id", "community_id"),
    CONSTRAINT fk_participant
        FOREIGN KEY ("participant_id") 
        REFERENCES "participant"("id")
        ON DELETE CASCADE,
    CONSTRAINT fk_community
        FOREIGN KEY ("community_id") 
        REFERENCES "community"("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "energytransfer" (
	"id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	"participant_from" UUID NOT NULL,
	"participant_to" UUID NOT NULL,
	"community" UUID NOT NULL,
	"energy_wh" NUMERIC(11,2) NOT NULL,
	"start" TIMESTAMP NOT NULL,
	"end" TIMESTAMP NOT NULL,
	CONSTRAINT fk_participant_from
		FOREIGN KEY ("participant_from")
		REFERENCES "participant"("id")
		ON DELETE CASCADE,
	CONSTRAINT fk_participant_to
		FOREIGN KEY ("participant_to")
		REFERENCES "participant"("id")
		ON DELETE CASCADE,
	CONSTRAINT fk_community
		FOREIGN KEY ("community")
		REFERENCES "community"("id")
		ON DELETE CASCADE
);
