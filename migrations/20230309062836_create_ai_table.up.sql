-- Add up migration script here
CREATE TABLE "prompts" (
    "id" uuid UNIQUE PRIMARY KEY NOT NULL,
    "prompt" varchar NOT NULL,
    "user_id" bigint NOT NULL,
    "guild_id" bigint NOT NULL,
    "created_at" date DEFAULT (now()),
    "updated_at" date DEFAULT (now())
);

CREATE TABLE "queries" (
    "id" uuid UNIQUE PRIMARY KEY NOT NULL,
    "query" varchar NOT NULL,
    "prompt_id" uuid NOT NULL,
    "user_id" bigint NOT NULL,
    "created_at" date DEFAULT (now()),
    "updated_at" date DEFAULT (now())
);

ALTER TABLE "prompts" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE "queries" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE "queries" ADD FOREIGN KEY ("prompt_id") REFERENCES "prompts" ("id");