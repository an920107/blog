CREATE TABLE IF NOT EXISTS "post_label" (
    "post_id" INTEGER NOT NULL,
    "label_id" INTEGER NOT NULL,
    PRIMARY KEY ("post_id", "label_id"),
    FOREIGN KEY ("post_id") REFERENCES "post" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("label_id") REFERENCES "label" ("id") ON DELETE CASCADE
);
