
DO $$ BEGIN
    CREATE TYPE JOB_STATUS AS ENUM ('rejected', 'accepted', 'pending');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

ALTER TABLE jobs
ADD status JOB_STATUS;

