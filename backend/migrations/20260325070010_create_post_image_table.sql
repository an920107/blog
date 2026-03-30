CREATE TABLE IF NOT EXISTS post_image (
    post_id INTEGER NOT NULL,
    image_id INTEGER NOT NULL,
    PRIMARY KEY (post_id, image_id),
    FOREIGN KEY ("post_id") REFERENCES "post" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("image_id") REFERENCES "image" ("id") ON DELETE CASCADE
);
