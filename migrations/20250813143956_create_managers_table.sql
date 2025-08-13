CREATE TABLE IF NOT EXISTS "manager" (
    "id" UUID NOT NULL UNIQUE,
    "community" UUID NOT NULL,
    PRIMARY KEY("id", "community")
);
