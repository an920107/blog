-- Triggers
ALTER TRIGGER update_post_updated_time ON "posts" RENAME TO update_posts_updated_time;
ALTER TRIGGER update_label_updated_time ON "labels" RENAME TO update_labels_updated_time;
ALTER TRIGGER update_image_updated_time ON "images" RENAME TO update_images_updated_time;
ALTER TRIGGER update_user_updated_time ON "users" RENAME TO update_users_updated_time;

-- Indexes
ALTER INDEX idx_post_semantic_id RENAME TO idx_posts_semantic_id;
ALTER INDEX idx_label_name RENAME TO idx_labels_name;
ALTER INDEX idx_user_email RENAME TO idx_users_email;
ALTER INDEX idx_user_source_id_issuer RENAME TO idx_users_source_id_issuer;

-- Primary key constraints (the backing index is renamed with the constraint)
ALTER TABLE "posts" RENAME CONSTRAINT post_pkey TO posts_pkey;
ALTER TABLE "labels" RENAME CONSTRAINT label_pkey TO labels_pkey;
ALTER TABLE "images" RENAME CONSTRAINT image_pkey TO images_pkey;
ALTER TABLE "users" RENAME CONSTRAINT user_pkey TO users_pkey;
ALTER TABLE "post_labels" RENAME CONSTRAINT post_label_pkey TO post_labels_pkey;
ALTER TABLE "post_images" RENAME CONSTRAINT post_image_pkey TO post_images_pkey;

-- Check constraints
ALTER TABLE "labels" RENAME CONSTRAINT label_color_check TO labels_color_check;

-- Foreign key constraints
ALTER TABLE "post_labels" RENAME CONSTRAINT post_label_post_id_fkey TO post_labels_post_id_fkey;
ALTER TABLE "post_labels" RENAME CONSTRAINT post_label_label_id_fkey TO post_labels_label_id_fkey;
ALTER TABLE "post_images" RENAME CONSTRAINT post_image_post_id_fkey TO post_images_post_id_fkey;
ALTER TABLE "post_images" RENAME CONSTRAINT post_image_image_id_fkey TO post_images_image_id_fkey;

-- Sequences
ALTER SEQUENCE post_id_seq RENAME TO posts_id_seq;
ALTER SEQUENCE label_id_seq RENAME TO labels_id_seq;
ALTER SEQUENCE image_id_seq RENAME TO images_id_seq;
ALTER SEQUENCE user_id_seq RENAME TO users_id_seq;
