CREATE TABLE IF NOT EXISTS "user" (
    "id" SERIAL PRIMARY KEY NOT NULL,
    "issuer" VARCHAR(100) NOT NULL,
    "source_id" VARCHAR(100) NOT NULL,
    "displayed_name" VARCHAR(100) NOT NULL,
    "email" VARCHAR(100) NOT NULL,
    "created_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS "idx_user_source_id_issuer"
ON "user" ("source_id", "issuer");

CREATE INDEX IF NOT EXISTS "idx_user_email"
ON "user" ("email");

CREATE OR REPLACE TRIGGER "update_user_updated_time"
BEFORE UPDATE ON "user"
FOR EACH ROW
EXECUTE FUNCTION update_updated_time_column();
