-- Add unique index to label name column
CREATE UNIQUE INDEX "idx_label_name" ON "label" ("name");
