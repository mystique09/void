CREATE TABLE IF NOT EXISTS "user" (
  user_id bigserial primary key,
  user_name varchar(30) not null unique,
  dc_id bigint not null unique,
  user_rank bigint not null default 0,
  user_exp bigint not null default 0,
  user_balance bigint not null default 0,
  -- inventory_id serial references "inventory"(inventory_id),
  created_at timestamp not null default now()
);
