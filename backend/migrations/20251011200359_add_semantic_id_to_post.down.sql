-- Drop the unique index on semantic_id
DROP INDEX IF EXISTS idx_post_semantic_id;

-- Remove the semantic_id column from post table
ALTER TABLE post DROP COLUMN IF EXISTS "semantic_id";
