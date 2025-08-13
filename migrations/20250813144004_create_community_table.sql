CREATE TABLE IF NOT EXISTS "Community" (
    "id" UUID NOT NULL UNIQUE,
    "entity" VARCHAR(255),
    PRIMARY KEY("id")
);
