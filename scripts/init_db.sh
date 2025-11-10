#!/usr/bin/env bash
set -x
set -eo pipefail

# Default settings for env vars
DB_PORT="${POSTGRES_PORT:=5430}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"
CONTAINER_NAME='postgres-zero2prod'

# Launch Postgres using docker

docker run \
    --env POSTGRES_USER="${SUPERUSER}" \
    --env POSTGRES_PWD="${SUPERUSER_PWD}" \
    --publish "${DB_PORT}":5432 \
    --detach \
    --name "${CONTAINER_NAME}" \
    postgres -N 1000 # max number of connections (for testing purposes)
