CREATE TABLE IF NOT EXISTS "post" (
    "id" SERIAL PRIMARY KEY NOT NULL,
    "title" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "preview_image_url" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    "published_time" TIMESTAMP,
    "deleted_time" TIMESTAMP,
    "created_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE TRIGGER "update_post_updated_time"
BEFORE UPDATE ON "post"
FOR EACH ROW
EXECUTE FUNCTION update_updated_time_column();
