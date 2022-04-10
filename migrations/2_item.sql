CREATE TABLE IF NOT EXISTS "item" (
  item_id serial primary key not null,
  item_name VARCHAR(20) unique not null,
  item_price bigint default 100,
  item_category VARCHAR(20) not null
  -- inventory_id serial references "inventory"(inventory_id)
);
