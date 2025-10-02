CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_role AS ENUM (
    'participant',
    'manager',
    'usermanager'
);

CREATE TABLE IF NOT EXISTS "user" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL UNIQUE,
    "password" VARCHAR(255) NOT NULL,
    PRIMARY KEY ("id")
);

CREATE INDEX IF NOT EXISTS "user_email_idx" ON "user" ("email");

CREATE TABLE IF NOT EXISTS "token" (
    "id" UUID NOT NULL,
    "user_id" UUID NOT NULL,
    "expiration" TIMESTAMPTZ NOT NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT fk_token_user
        FOREIGN KEY ("user_id")
        REFERENCES "user"("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "community" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "name" VARCHAR(255) NOT NULL UNIQUE,
    "image" UUID NOT NULL DEFAULT gen_random_uuid(),
    PRIMARY KEY ("id")
);

CREATE INDEX IF NOT EXISTS "community_name_idx" ON "community" ("name");

CREATE TABLE IF NOT EXISTS "user_community" (
    "user_id" UUID NOT NULL,
    "community_id" UUID NOT NULL,
    "role" user_role NOT NULL,
    PRIMARY KEY ("user_id", "community_id"),
    CONSTRAINT fk_user_community
        FOREIGN KEY ("user_id")
        REFERENCES "user"("id")
        ON DELETE CASCADE,
    CONSTRAINT fk_community_user
        FOREIGN KEY ("community_id")
        REFERENCES "community"("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "energypool" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "user_id" UUID NOT NULL,
    "community_id" UUID NOT NULL,
    "generated" NUMERIC(11,2) NOT NULL CHECK ("generated" >= 0),
    "consumed" NUMERIC(11,2) NOT NULL CHECK ("consumed" >= 0),
    "consumer_price" NUMERIC(11,2) NOT NULL CHECK ("generated" >= 0),
    "seller_price" NUMERIC(11,2) NOT NULL CHECK ("consumed" >= 0),
    "start" TIMESTAMP NOT NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT fk_energypool_user
        FOREIGN KEY ("user_id")
        REFERENCES "user"("id")
        ON DELETE CASCADE,
    CONSTRAINT fk_energypool_community
        FOREIGN KEY ("community_id")
        REFERENCES "community"("id")
        ON DELETE CASCADE
);
