CREATE TABLE IF NOT EXISTS "key" (
    "id" VARCHAR(255) NOT NULL,
    "user_id" UUID NOT NULL,
    "hashed_password" VARCHAR(255),
    PRIMARY KEY ("id"),
    CONSTRAINT fk_key_user
        FOREIGN KEY ("user_id")
        REFERENCES "user"("id")
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "key_user_id_idx" ON "key" ("user_id");

INSERT INTO "key" ("id", "user_id", "hashed_password")
SELECT CONCAT('email:', "email"), "id", "password"
FROM "user"
WHERE "password" IS NOT NULL;

ALTER TABLE "user" DROP COLUMN "password";