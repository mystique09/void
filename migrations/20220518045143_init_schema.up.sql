-- SQL dump generated using DBML (dbml-lang.org)
-- Database: PostgreSQL
-- Generated at: 2022-05-18T08:03:03.373Z

CREATE TYPE "item_category" AS ENUM (
  'consumable',
  'upgrade',
  'weapon',
  'essential',
  'box'
);

CREATE TYPE "item_name" AS ENUM (
  'common_box',
  'rare_box',
  'epic_box',
  'mythic_box',
  'legendary_box',
  'apple',
  'soup',
  'nuke',
  'knife'
);

CREATE TABLE "profile" (
  "id" SERIAL UNIQUE PRIMARY KEY,
  "uid" varchar UNIQUE NOT NULL,
  "username" varchar UNIQUE NOT NULL,
  "rank" bigint NOT NULL DEFAULT 1,
  "exp" int NOT NULL DEFAULT 0,
  "wallet" bigint NOT NULL DEFAULT 1000,
  "bank" bigint NOT NULL DEFAULT 0,
  "diamond" bigint NOT NULL DEFAULT 0,
  "guild_id" varchar NOT NULL,
  "created_at" timestamptz DEFAULT (now()),
  "updated_at" timestamptz DEFAULT (now())
);

CREATE TABLE "item" (
  "id" SERIAL,
  "item_owner" varchar NOT NULL,
  "name" item_name NOT NULL,
  "category" item_category NOT NULL,
  "created_at" timestamptz DEFAULT (now()),
  "updated_at" timestamptz DEFAULT (now())
);

COMMENT ON COLUMN "profile"."wallet" IS 'The user wallet balance.';

COMMENT ON COLUMN "profile"."bank" IS 'The user"s bank.';

COMMENT ON COLUMN "profile"."diamond" IS 'Diamond for premium items.';

ALTER TABLE "item" ADD FOREIGN KEY ("item_owner") REFERENCES "profile" ("uid");
