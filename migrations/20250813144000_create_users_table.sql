CREATE TABLE IF NOT EXISTS "User" (
    "id" UUID NOT NULL UNIQUE,
    "community" UUID NOT NULL,
    PRIMARY KEY("id")
);
