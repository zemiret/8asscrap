.PHONY: dbstart dbstart-fg dbstop run-once run

dbstart:
	cd compose; \
	docker compose up -d

dbstart-fg:
	docker compose -f docker-compose-db.yml up

dbstop:
	docker compose -f docker-compose-db.yml down

run:
	cargo run .

