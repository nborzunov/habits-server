CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Begin a transaction
BEGIN;

-- Rename the column 'title' to 'name'
ALTER TABLE habits RENAME COLUMN title TO name;

-- Remove columns that are not present in the new schema
ALTER TABLE habits
    DROP COLUMN IF EXISTS periodicity,
    DROP COLUMN IF EXISTS periodicity_value,
    DROP COLUMN IF EXISTS goal_type,
    DROP COLUMN IF EXISTS allow_skip,
    DROP COLUMN IF EXISTS allow_partial_completion,
    DROP COLUMN IF EXISTS allow_over_goal_completion,
    DROP COLUMN IF EXISTS can_be_finished,
    DROP COLUMN IF EXISTS total_goal;

-- Add new columns if they do not exist
DO $$ 
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='habits' AND column_name='color') THEN
        ALTER TABLE habits ADD COLUMN color VARCHAR NOT NULL;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='habits' AND column_name='icon') THEN
        ALTER TABLE habits ADD COLUMN icon VARCHAR NOT NULL;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='habits' AND column_name='amount') THEN
        ALTER TABLE habits ADD COLUMN amount INTEGER NOT NULL;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='habits' AND column_name='frequency_type') THEN
        ALTER TABLE habits ADD COLUMN frequency_type VARCHAR NOT NULL;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='habits' AND column_name='frequency_amount') THEN
        ALTER TABLE habits ADD COLUMN frequency_amount JSONB NOT NULL;
    END IF;
END $$;

-- Commit the transaction
COMMIT;