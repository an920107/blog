-- Add migration script here

CREATE TABLE "post" (
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

CREATE TABLE "label" (
    "id" SERIAL PRIMARY KEY NOT NULL,
    "name" TEXT NOT NULL,
    "color" BIGINT NOT NULL CHECK ("color" >= 0 AND "color" <= 4294967295),
    "deleted_time" TIMESTAMP,
    "created_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "post_label" (
    "post_id" INTEGER NOT NULL,
    "label_id" INTEGER NOT NULL,
    PRIMARY KEY ("post_id", "label_id"),
    FOREIGN KEY ("post_id") REFERENCES "post" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("label_id") REFERENCES "label" ("id") ON DELETE CASCADE
);


-- Auto update `updated_time` trigger

CREATE FUNCTION update_updated_time_column() RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_time = CURRENT_TIMESTAMP;
    return NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER "update_post_updated_time"
BEFORE UPDATE ON "post"
FOR EACH ROW
EXECUTE FUNCTION update_updated_time_column();

CREATE TRIGGER "update_label_updated_time"
BEFORE UPDATE ON "label"
FOR EACH ROW
EXECUTE FUNCTION update_updated_time_column();

