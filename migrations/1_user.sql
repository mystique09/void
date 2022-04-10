CREATE TABLE IF NOT EXISTS "user" (
  user_id serial primary key,
  dc_id bigint not null unique,
  user_rank integer default 0,
  user_balance bigint default 0,
  -- inventory_id serial references "inventory"(inventory_id),
  created_at timestamp not null default now()
);
