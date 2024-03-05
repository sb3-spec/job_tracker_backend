CREATE TABLE IF NOT EXISTS users (
    supabase_id varchar(255) NOT NULL PRIMARY KEY,
    email varchar(255) NOT NULL,
    first_name varchar(255) NOT NULL,
    last_name varchar(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS jobs (
    id bigserial PRIMARY KEY,
    title varchar(255) NOT NULL,
    company varchar(255) NOT NULL,
    application_link varchar(255) NOT NULL,
    ctime timestamp without time zone NOT NULL DEFAULT now(),
    user_id varchar(255) NOT NULL,

    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES users(supabase_id)
)