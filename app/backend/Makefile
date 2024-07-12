.PHONY: dbstart dbstart-fg

dbstart:
	docker compose -f docker-compose-db.yml up -d

dbstart-fg:
	docker compose -f docker-compose-db.yml up

dbstop:
	docker compose -f docker-compose-db.yml down

run:
	cargo run .
