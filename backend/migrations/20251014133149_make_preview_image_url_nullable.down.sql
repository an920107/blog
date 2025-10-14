-- Revert post.preview_image_url to NOT NULL
ALTER TABLE "post" ALTER COLUMN "preview_image_url" SET NOT NULL;
