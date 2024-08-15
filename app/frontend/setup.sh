#!/usr/bin/env sh

# This script will generate required env files

set -eu

create_envs() {
  echo "** Creating environment file .env **"

cat <<EOF > .env
PUBLIC_API_BASE_URL=http://localhost:8080/api/v1
EOF

}

create_envs

