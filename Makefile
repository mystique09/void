setup:
	cp .sample.env .env

migrate:
	cargo sqlx migrate run

reset:
	cargo sqlx database reset

revert:
	cargo sqlx migrate revert

add:
	cargo sqlx migrate add -r $(des)

.PHONY: setup migrate revert add reset
