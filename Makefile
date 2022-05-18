setup:
	cp .sample.env .env

migrate:
	sqlx migrate run

reset:
	sqlx database reset

revert:
	sqlx migrate revert

add:
	sqlx migrate add -r $(des)

.PHONY: setup migrate revert add reset
