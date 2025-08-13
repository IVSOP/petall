CREATE TABLE IF NOT EXISTS "Manager" (
    "id" UUID NOT NULL UNIQUE,
    "community" UUID NOT NULL,
    PRIMARY KEY("id", "community")
);
