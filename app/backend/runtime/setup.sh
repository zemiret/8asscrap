#!/usr/bin/env sh

# This script will generate env files and redash database.

set -eu

create_envs() {
  echo "** Creating environment files **"

  # Minimum mandatory values (when not just developing)
  REDASH_COOKIE_SECRET=$(pwgen -1s 32)
  REDASH_SECRET_KEY=$(pwgen -1s 32)
  REDASH_PG_PASSWORD=$(pwgen -1s 32 | tr '[:upper:]' '[:lower:]'                                                                                      )
  REDASH_DATABASE_URL="postgresql://postgres:${REDASH_PG_PASSWORD}@postgres/postgres"
  MONGO_INITDB_ROOT_PASSWORD=$(pwgen -1s 32)

  echo "Generating env_compose"

  cat <<EOF > env_compose
PYTHONUNBUFFERED=0
REDASH_LOG_LEVEL=INFO
REDASH_REDIS_URL=redis://redis:6379/0
REDASH_COOKIE_SECRET=$REDASH_COOKIE_SECRET
REDASH_SECRET_KEY=$REDASH_SECRET_KEY
POSTGRES_PASSWORD=$REDASH_PG_PASSWORD
REDASH_DATABASE_URL=$REDASH_DATABASE_URL
REDASH_ENFORCE_CSRF=true
REDASH_GUNICORN_TIMEOUT=60
MONGO_INITDB_ROOT_USERNAME=root
MONGO_INITDB_ROOT_PASSWORD=$MONGO_INITDB_ROOT_PASSWORD 
EOF

  echo "Generating env_app. REMEMBER TO FILL THIS ONE IN!"

cat <<EOF > env_app
APP_DEBUG=<false or true>
APP_DB_USER=root
APP_DB_PASSWORD=$MONGO_INITDB_ROOT_PASSWORD
APP_SIDEXPORTER_USERNAME='<8A_USERNAME_HERE>'
APP_SIDEXPORTER_PASSWORD='<8A_PASSWORD_HERE>' 
EOF
}

redash_createdb() {
    echo "** Initialising Redash database **"
	docker compose -f compose-redash.yml run --rm server create_db
	docker compose -f compose-redash.yml down
}

create_envs
redash_createdb

