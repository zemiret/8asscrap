.PHONY: runwatch run dbstart dbstart-fg dbstop redashstart redashstart-fg redashstop rundocker rundocker-fg rundocker-stop setup docker-build

setup:
	cd runtime; \
	./setup.sh \
	cd .. \
	$(MAKE) docker-build

# runs the app locally, watches for changes and recompiles
# (will not work if the current user does not own all the files, even the ignored ones)
runwatch:
	cargo watch --ignore 'runtime/**' -x 'run -- --envfile runtime/env_app_local'

# runs the app locally 
run:
	cargo run -- --envfile runtime/env_app_local

docker-build:
	docker build -t backend .

rundocker-fg:
	cd runtime; \
	docker compose up

rundocker:
	cd runtime; \
	docker compose up -d

rundocker-stop:
	cd runtime; \
	docker compose stop

dbstart:
	cd runtime; \
	docker compose -f compose-db.yml up -d

dbstart-fg:
	cd runtime; \
	docker compose -f compose-db.yml up

dbstop:
	cd runtime; \
	docker compose -f compose-db.yml down

redashstart:
	cd runtime; \
	docker compose -f compose-redash.yml up -d

redashstart-fg:
	cd runtime; \
	docker compose -f compose-redash.yml up

redashstop:
	cd runtime; \
	docker compose -f compose-redash.yml down


