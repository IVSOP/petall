CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE participant_role AS ENUM (
    'user',
    'manager',
    'usermanager'
);

CREATE TABLE IF NOT EXISTS "supplier" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "email" VARCHAR(255) NOT NULL UNIQUE,
    "name" VARCHAR(255) NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE IF NOT EXISTS "participant" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL UNIQUE,
    "password" VARCHAR(255) NOT NULL,
    "supplier" UUID NOT NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT fk_participant_supplier
        FOREIGN KEY ("supplier") 
        REFERENCES "supplier"("id")
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "participant_email_idx" ON "participant" ("email");

CREATE TABLE IF NOT EXISTS "community" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "name" VARCHAR(255) NOT NULL UNIQUE,
    "image" UUID NOT NULL DEFAULT gen_random_uuid(),
    PRIMARY KEY ("id")
);

CREATE INDEX IF NOT EXISTS "community_name_idx" ON "community" ("name");

CREATE TABLE IF NOT EXISTS "participant_community" (
    "participant" UUID NOT NULL,
    "community" UUID NOT NULL,
    "role" participant_role NOT NULL,
    PRIMARY KEY ("participant", "community"),
    CONSTRAINT fk_participant_community
        FOREIGN KEY ("participant") 
        REFERENCES "participant"("id")
        ON DELETE CASCADE,
    CONSTRAINT fk_community_participant
        FOREIGN KEY ("community") 
        REFERENCES "community"("id")
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "energypool" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "participant" UUID NOT NULL,
    "community" UUID NOT NULL,
    "generated" NUMERIC(11,2) NOT NULL CHECK ("generated" >= 0),
    "consumed" NUMERIC(11,2) NOT NULL CHECK ("consumed" >= 0),
    "consumer_price" NUMERIC(11,2) NOT NULL CHECK ("generated" >= 0),
    "seller_price" NUMERIC(11,2) NOT NULL CHECK ("consumed" >= 0),
    "start" TIMESTAMP NOT NULL,
    PRIMARY KEY ("id"),
    CONSTRAINT fk_energypool_participant
        FOREIGN KEY ("participant")
        REFERENCES "participant"("id")
        ON DELETE CASCADE,
    CONSTRAINT fk_energypool_community
        FOREIGN KEY ("community")
        REFERENCES "community"("id")
        ON DELETE CASCADE
);
