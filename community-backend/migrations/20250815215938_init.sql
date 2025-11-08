CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE auth_provider AS ENUM (
    'email',
    'github',
    'google'
);

CREATE TABLE IF NOT EXISTS "user" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL UNIQUE,
    "is_admin" BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY ("id")
);

CREATE INDEX IF NOT EXISTS "user_email_idx" ON "user" ("email");

CREATE TABLE IF NOT EXISTS "key" (
    "provider" auth_provider NOT NULL,
    "id" VARCHAR(255) NOT NULL,
    "user_id" UUID NOT NULL,
    "hashed_password" VARCHAR(255),
    PRIMARY KEY ("provider", "id"),
    CONSTRAINT fk_key_user
        FOREIGN KEY ("user_id")
        REFERENCES "user"("id")
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "key_user_id_idx" ON "key" ("user_id");
CREATE INDEX IF NOT EXISTS "key_provider_id_idx" ON "key" ("provider", "id");

CREATE TABLE IF NOT EXISTS "session" (
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "expiration" TIMESTAMPTZ NOT NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT fk_session_user
        FOREIGN KEY ("user_id")
        REFERENCES "user"("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "community" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "name" VARCHAR(255) NOT NULL UNIQUE,
    "description" TEXT NOT NULL,
    "image" VARCHAR(255),
    PRIMARY KEY ("id")
);

CREATE INDEX IF NOT EXISTS "community_name_idx" ON "community" ("name");

CREATE TABLE IF NOT EXISTS "community_user" (
    "user_id" UUID NOT NULL,
    "community_id" UUID NOT NULL,
    PRIMARY KEY ("user_id", "community_id"),
    CONSTRAINT fk_community_user_user
        FOREIGN KEY ("user_id")
        REFERENCES "user"("id")
        ON DELETE CASCADE,
    CONSTRAINT fk_community_user_community
        FOREIGN KEY ("community_id")
        REFERENCES "community"("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "community_manager" (
    "user_id" UUID NOT NULL,
    "community_id" UUID NOT NULL,
    PRIMARY KEY ("user_id", "community_id"),
    CONSTRAINT fk_community_manager_user
        FOREIGN KEY ("user_id")
        REFERENCES "user"("id")
        ON DELETE CASCADE,
    CONSTRAINT fk_community_manager_community
        FOREIGN KEY ("community_id")
        REFERENCES "community"("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "energy_record" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "user_id" UUID NOT NULL,
    "community_id" UUID NOT NULL,
    "generated" NUMERIC(11,4) NOT NULL CHECK ("generated" >= 0),
    "consumed" NUMERIC(11,4) NOT NULL CHECK ("consumed" >= 0),
    "consumer_price" NUMERIC(11,4) NOT NULL CHECK ("consumer_price" >= 0),
    "seller_price" NUMERIC(11,4) NOT NULL CHECK ("seller_price" >= 0),
    "start" TIMESTAMP NOT NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT fk_energy_record_user
        FOREIGN KEY ("user_id")
        REFERENCES "user"("id")
        ON DELETE CASCADE,
    CONSTRAINT fk_energy_record_community
        FOREIGN KEY ("community_id")
        REFERENCES "community"("id")
        ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_energy_user_community_start
ON energy_record (user_id, community_id, start);
