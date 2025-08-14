CREATE TABLE IF NOT EXISTS "manager" (
	"id" UUID NOT NULL UNIQUE,
	"name" VARCHAR(255) NOT NULL,
	"email" VARCHAR(255) NOT NULL UNIQUE,
	PRIMARY KEY("id")
);

CREATE INDEX IF NOT EXISTS "manager_email_idx" ON "manager" ("email");
