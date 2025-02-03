DROP TABLE api_requests;
DROP TABLE user_api_keys;
DROP TABLE application_api_keys;
DROP TABLE applications;
DROP FUNCTION IF EXISTS execute_org_query(BIGINT, text) CASCADE;
DROP FUNCTION IF EXISTS execut_org_user_query(BIGINT, text) CASCADE;
