CREATE TABLE IF NOT EXISTS "label" (
    "id" SERIAL PRIMARY KEY NOT NULL,
    "name" TEXT NOT NULL,
    "color" BIGINT NOT NULL CHECK ("color" >= 0 AND "color" <= 4294967295),
    "deleted_time" TIMESTAMP,
    "created_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE TRIGGER "update_label_updated_time"
BEFORE UPDATE ON "label"
FOR EACH ROW
EXECUTE FUNCTION update_updated_time_column();
