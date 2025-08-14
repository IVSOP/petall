CREATE TABLE IF NOT EXISTS "user-community" (
	"user_id" UUID NOT NULL,
	"community_id" UUID NOT NULL,
	PRIMARY KEY("user_id", "community_id")
);
