CREATE TABLE IF NOT EXISTS "manager" (
	"id" UUID NOT NULL UNIQUE,
	"name" VARCHAR(255) NOT NULL,
	"email" VARCHAR(255) NOT NULL UNIQUE,
	PRIMARY KEY("id")
);

CREATE INDEX IF NOT EXISTS "manager_email_idx" ON "manager" ("email");

CREATE TABLE IF NOT EXISTS "user" (
	"id" UUID NOT NULL UNIQUE,
	"name" VARCHAR(255) NOT NULL,
	"email" VARCHAR(255) NOT NULL UNIQUE,
	PRIMARY KEY("id")
);

CREATE INDEX IF NOT EXISTS "user_email_idx" ON "user" ("email");

CREATE TABLE IF NOT EXISTS "community" (
	"id" UUID NOT NULL UNIQUE,
	"entity" VARCHAR(255) NOT NULL UNIQUE,
	PRIMARY KEY("id")
);

CREATE INDEX IF NOT EXISTS "community_entity_idx" ON "community" ("entity");

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

CREATE TABLE IF NOT EXISTS "user-community" (
	"user_id" UUID NOT NULL,
	"community_id" UUID NOT NULL,
	PRIMARY KEY("user_id", "community_id")
);

CREATE TABLE IF NOT EXISTS "manager-community" (
	"manager_id" UUID NOT NULL,
	"community_id" UUID NOT NULL,
	PRIMARY KEY("manager_id", "community_id")
);

ALTER TABLE "energytransfer"
ADD FOREIGN KEY("user_to") REFERENCES "user"("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;

ALTER TABLE "energytransfer"
ADD FOREIGN KEY("community") REFERENCES "community"("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;

ALTER TABLE "user-community"
ADD FOREIGN KEY("user_id") REFERENCES "user"("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;

ALTER TABLE "user-community"
ADD FOREIGN KEY("community_id") REFERENCES "community"("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;

ALTER TABLE "manager-community"
ADD FOREIGN KEY("manager_id") REFERENCES "manager"("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;

ALTER TABLE "manager-community"
ADD FOREIGN KEY("community_id") REFERENCES "community"("id")
ON UPDATE NO ACTION ON DELETE NO ACTION;
