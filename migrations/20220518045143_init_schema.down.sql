-- Add down migration script here
DROP TABLE IF EXISTS "roles";
DROP TABLE IF EXISTS "user_roles";
DROP TABLE IF EXISTS "users";
DROP TABLE IF EXISTS "keywords";
DROP TABLE IF EXISTS "rss_feeds";
DROP TYPE IF EXISTS "ResponseType";
DROP TYPE IF EXISTS "ResponseMode";
DROP TYPE IF EXISTS "RefreshInterval";