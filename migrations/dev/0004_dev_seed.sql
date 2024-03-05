INSERT INTO users (supabase_id, email, first_name, last_name) VALUES ('test_token', 'test_email@email.com', 'test_first_name', 'test_last_name');
INSERT INTO users (supabase_id, email, first_name, last_name) VALUES ('test_delete', 'test_email@email.com', 'test_first_name', 'test_last_name');


INSERT INTO jobs (title, company, application_link, user_id, status) VALUES ('test_job', 'test_company', 'test_application_link.com', 'test_token', 'pending');
INSERT INTO jobs (title, company, application_link, user_id, status) VALUES ('test_delete', 'test_delete', 'test_delete.com', 'test_token', 'pending');