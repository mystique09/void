CREATE TABLE IF NOT EXISTS "inventory" (
  inventory_id serial primary key not null,
  user_id uuid references "user"(user_id) on delete cascade,
);
