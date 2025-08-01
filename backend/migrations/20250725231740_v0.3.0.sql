CREATE TABLE "image" (
    "id" SERIAL PRIMARY KEY NOT NULL,
    "mime_type" VARCHAR(100) NOT NULL,
    "deleted_time" TIMESTAMP,
    "created_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "user" (
    "id" SERIAL PRIMARY KEY NOT NULL,
    "issuer" VARCHAR(100) NOT NULL,
    "source_id" VARCHAR(100) NOT NULL,
    "displayed_name" VARCHAR(100) NOT NULL,
    "email" VARCHAR(100) NOT NULL,
    "created_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX "user_source_id_issuer_key" ON "user" ("source_id", "issuer");
CREATE INDEX "user_email_key" ON "user" HASH ("email");

CREATE TRIGGER "update_image_updated_time"
BEFORE UPDATE ON "image"
FOR EACH ROW
EXECUTE FUNCTION update_updated_time_column();

CREATE TRIGGER "update_user_updated_time"
BEFORE UPDATE ON "user"
FOR EACH ROW
EXECUTE FUNCTION update_updated_time_column();
