-- Make post.preview_image_url nullable
ALTER TABLE "post" ALTER COLUMN "preview_image_url" DROP NOT NULL;
