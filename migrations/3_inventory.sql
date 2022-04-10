CREATE TABLE IF NOT EXISTS "inventory" (
  inventory_id serial primary key not null,
  user_id serial references "user"(user_id) on delete cascade
);
