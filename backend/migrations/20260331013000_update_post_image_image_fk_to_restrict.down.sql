ALTER TABLE post_image
DROP CONSTRAINT IF EXISTS post_image_image_id_fkey;

ALTER TABLE post_image
ADD CONSTRAINT post_image_image_id_fkey
FOREIGN KEY (image_id)
REFERENCES image(id)
ON DELETE CASCADE;
