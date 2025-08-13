CREATE TABLE IF NOT EXISTS "community" (
    "id" UUID NOT NULL UNIQUE,
    "entity" VARCHAR(255),
    PRIMARY KEY("id")
);
