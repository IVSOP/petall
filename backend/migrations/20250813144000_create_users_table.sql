CREATE TABLE IF NOT EXISTS "user" (
    "id" UUID NOT NULL UNIQUE,
    "community" UUID NOT NULL,
    PRIMARY KEY("id")
);
