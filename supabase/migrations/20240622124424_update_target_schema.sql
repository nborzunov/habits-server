CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Begin a transaction
BEGIN;

-- Rename the column 'value' to 'amount'
ALTER TABLE targets RENAME COLUMN value TO amount;

-- Remove columns that are not present in the new schema
ALTER TABLE targets
    DROP COLUMN IF EXISTS target_type;

-- Commit the transaction
COMMIT;