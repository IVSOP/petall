CREATE TABLE IF NOT EXISTS "energytransfer" (
	"user_from" UUID NOT NULL,
	"user_to" UUID NOT NULL,
	"community" UUID NOT NULL,
	"energy_Wh" DECIMAL NOT NULL,
	"start" TIMESTAMP NOT NULL,
	"end" TIMESTAMP NOT NULL,
	"id" UUID NOT NULL UNIQUE,
	PRIMARY KEY("id")
);
