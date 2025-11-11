#!/usr/bin/env bash
set -x
set -eo pipefail


if ! [ -x "$(command -v sqlx)" ]; then
echo >&2 "Error: sqlx is not installed."
echo >&2 "Use:"
echo >&2 " cargo install --version='~0.8' sqlx-cli \
--no-default-features --features rustls,postgres" echo >&2 "to install it."
exit 1
fi

# Default settings for env vars
DB_PORT="${POSTGRES_PORT:=5430}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"
CONTAINER_NAME='postgres-zero2prod'

# Launch Postgres using docker
docker run \
    --env POSTGRES_USER="${SUPERUSER}" \
    --env POSTGRES_PASSWORD="${SUPERUSER_PWD}" \
    --health-cmd="pg_isready -U ${SUPERUSER} || exit 1" \
    --health-interval=1s \
    --health-timeout=5s \
    --health-retries=5 \
    --publish "${DB_PORT}":5432 \
    --detach \
    --name "${CONTAINER_NAME}" \
    postgres -N 1000

is_ready() {
    [ "$(docker inspect -f "{{.State.Health.Status}}" "${CONTAINER_NAME}")" = "healthy" ]
}

# Wait for Postgres to be ready to accept connections
until is_ready; do
    >&2 echo "Postgres still unavailable - sleeping"
    sleep 1
done
>&2 echo "Postgres is up and running on port ${DB_PORT}!"

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


# using sqlx-cli to manage database migrations.
# Export DATABASE_URL for sqlx commands
export DATABASE_URL=postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}

# Create the database if it doesn't exist
sqlx database create

# Run all pending migrations
# This ensures the database schema is always up-to-date when the script completes
# Idempotent: safe to run multiple times (only applies unapplied migrations)
sqlx migrate run
