CREATE TABLE "image" (
    "id" SERIAL PRIMARY KEY NOT NULL,
    "mime_type" VARCHAR(100) NOT NULL,
    "deleted_time" TIMESTAMP,
    "created_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER "update_image_updated_time"
BEFORE UPDATE ON "image"
FOR EACH ROW
EXECUTE FUNCTION update_updated_time_column();
