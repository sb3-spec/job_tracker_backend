CREATE TYPE JOB_STATUS AS ENUM ('rejected', 'accepted', 'pending');

ALTER TABLE jobs
ADD status JOB_STATUS;