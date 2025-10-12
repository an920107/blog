ALTER TABLE post ADD COLUMN "semantic_id" VARCHAR(100) NOT NULL DEFAULT '';

-- Update existing records to use their id as semantic_id with _id prefix
UPDATE post SET semantic_id = '_' || id::VARCHAR WHERE semantic_id = '';

-- Create unique index on semantic_id
CREATE UNIQUE INDEX idx_post_semantic_id ON post (semantic_id);
