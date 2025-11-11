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


# WARNING: By default, Postgres launches with a superuser named postgres, owner of a default database named postgres.
# It is a good practice to avoid using the superuser for our applications, as it has too many privileges.
# NOTE: create a dedicated application user, with enough privileges to create its own database
APP_USER="${APP_USER:=app}"
APP_USER_PWD="${APP_USER_PWD:=secret}"
APP_DB_NAME="${APP_DB_NAME:=newsletter}"

# Execute SQL command as postgres superuser
run_sql() {
    docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "$1"
}

# CREATE THE APPLICATION USER
run_sql "CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"

# GRANT DB PRIVILEGES TO THE APP USER
run_sql "ALTER USER ${APP_USER} CREATEDB;"
