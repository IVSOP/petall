CREATE TABLE IF NOT EXISTS "energytransfer" (
    "id" UUID NOT NULL UNIQUE,
    "user_from" UUID NOT NULL,
    "user_to" UUID NOT NULL,
    "community" UUID NOT NULL,
    "energy_wh" DECIMAL NOT NULL,
    "start" TIMESTAMP NOT NULL,
    "end" TIMESTAMP NOT NULL,
    PRIMARY KEY("id")
);
