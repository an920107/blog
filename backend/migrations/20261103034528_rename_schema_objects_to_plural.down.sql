-- Sequences
ALTER SEQUENCE posts_id_seq RENAME TO post_id_seq;
ALTER SEQUENCE labels_id_seq RENAME TO label_id_seq;
ALTER SEQUENCE images_id_seq RENAME TO image_id_seq;
ALTER SEQUENCE users_id_seq RENAME TO user_id_seq;

-- Foreign key constraints
ALTER TABLE "post_images" RENAME CONSTRAINT post_images_image_id_fkey TO post_image_image_id_fkey;
ALTER TABLE "post_images" RENAME CONSTRAINT post_images_post_id_fkey TO post_image_post_id_fkey;
ALTER TABLE "post_labels" RENAME CONSTRAINT post_labels_label_id_fkey TO post_label_label_id_fkey;
ALTER TABLE "post_labels" RENAME CONSTRAINT post_labels_post_id_fkey TO post_label_post_id_fkey;

-- Check constraints
ALTER TABLE "labels" RENAME CONSTRAINT labels_color_check TO label_color_check;

-- Primary key constraints (the backing index is renamed with the constraint)
ALTER TABLE "post_images" RENAME CONSTRAINT post_images_pkey TO post_image_pkey;
ALTER TABLE "post_labels" RENAME CONSTRAINT post_labels_pkey TO post_label_pkey;
ALTER TABLE "users" RENAME CONSTRAINT users_pkey TO user_pkey;
ALTER TABLE "images" RENAME CONSTRAINT images_pkey TO image_pkey;
ALTER TABLE "labels" RENAME CONSTRAINT labels_pkey TO label_pkey;
ALTER TABLE "posts" RENAME CONSTRAINT posts_pkey TO post_pkey;

-- Indexes
ALTER INDEX idx_users_source_id_issuer RENAME TO idx_user_source_id_issuer;
ALTER INDEX idx_users_email RENAME TO idx_user_email;
ALTER INDEX idx_labels_name RENAME TO idx_label_name;
ALTER INDEX idx_posts_semantic_id RENAME TO idx_post_semantic_id;

-- Triggers
ALTER TRIGGER update_users_updated_time ON "users" RENAME TO update_user_updated_time;
ALTER TRIGGER update_images_updated_time ON "images" RENAME TO update_image_updated_time;
ALTER TRIGGER update_labels_updated_time ON "labels" RENAME TO update_label_updated_time;
ALTER TRIGGER update_posts_updated_time ON "posts" RENAME TO update_post_updated_time;
