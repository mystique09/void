CREATE TYPE "ResponseType" AS ENUM (
  'SingleLine',
  'MultiLine',
  'Media'
);

CREATE TYPE "ResponseMode" AS ENUM (
  'Regular',
  'DirectMessage'
);

CREATE TYPE "RefreshInterval" AS ENUM (
  'Hourly',
  'Daily',
  'Weekly',
  'Monthly',
  'Annualy'
);

CREATE TABLE "users" (
  "id" bigint UNIQUE PRIMARY KEY NOT NULL,
  "nickname" varchar NOT NULL,
  "created_at" date DEFAULT (now()),
  "updated_at" date DEFAULT (now())
);

CREATE TABLE "user_roles" (
  "user_id" bigint PRIMARY KEY,
  "role_id" bigint
);

CREATE TABLE "roles" (
  "id" bigint UNIQUE PRIMARY KEY NOT NULL,
  "name" varchar NOT NULL,
  "created_at" date DEFAULT (now()),
  "updated_at" date DEFAULT (now())
);

CREATE TABLE "keywords" (
  "id" bigint UNIQUE PRIMARY KEY NOT NULL,
  "word" varchar NOT NULL,
  "guild_id" bigint NOT NULL,
  "response" varchar NOT NULL,
  "response_type" "ResponseType" NOT NULL DEFAULT 'SingleLine',
  "response_mode" "ResponseMode" NOT NULL DEFAULT 'Regular',
  "created_at" date DEFAULT (now()),
  "updated_at" date DEFAULT (now())
);

CREATE TABLE "rss_feeds" (
  "id" bigint UNIQUE PRIMARY KEY NOT NULL,
  "feed_link" varchar NOT NULL,
  "guild_id" bigint NOT NULL,
  "channel_id" bigint NOT NULL,
  "refresh_interval" "RefreshInterval" DEFAULT 'Hourly',
  "created_at" date DEFAULT (now()),
  "updated_at" date DEFAULT (now())
);

COMMENT ON COLUMN "users"."id" IS 'the id of user in discord(e.g, "421609061495341066").';

COMMENT ON COLUMN "users"."nickname" IS 'discord nickname(do we need to store the previous nicknames? what if the user will change nickname?)';

COMMENT ON TABLE "user_roles" IS 'roles of user contains user and role id';

COMMENT ON TABLE "roles" IS 'Server roles';

COMMENT ON COLUMN "roles"."id" IS 'the id of role in the server, so we can mention it via <@&role-id>';

COMMENT ON COLUMN "roles"."name" IS 'role name lmao';

COMMENT ON TABLE "keywords" IS 'for auto responder';

COMMENT ON COLUMN "keywords"."word" IS 'the word to detect';

COMMENT ON COLUMN "keywords"."response" IS 'the response, can be a gif XD';

COMMENT ON COLUMN "keywords"."response_type" IS 'whether the response is single line(default), multiline or media.';

COMMENT ON COLUMN "keywords"."response_mode" IS 'where to send the response, dm or regular(default)';

COMMENT ON TABLE "rss_feeds" IS 'for rss feeds';

COMMENT ON COLUMN "rss_feeds"."feed_link" IS 'the link of rss feed';

COMMENT ON COLUMN "rss_feeds"."channel_id" IS 'what channel does the feeds must be sent';

COMMENT ON COLUMN "rss_feeds"."refresh_interval" IS 'how often the feed should be sent';

ALTER TABLE "user_roles" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "user_roles" ADD FOREIGN KEY ("role_id") REFERENCES "roles" ("id");