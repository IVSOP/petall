CREATE TABLE IF NOT EXISTS "manager-community" (
	"manager_id" UUID NOT NULL UNIQUE,
	"community_id" UUID NOT NULL,
	PRIMARY KEY("manager_id", "community_id")
);
