#!/bin/bash
#
# Resets the database undoing all migrations

# Read the .env file
source .env

# Check if DEV_MODE is true
if [ "$PUBLIC_DEV_MODE" != "true" ]; then
    echo "Error: DEV_MODE is not set to true"
    exit 1
fi

# Function to run a SQL query
run_query() {
    psql "$DATABASE_URL" -c "$1"
}

while true; do
    output=$(sqlx migrate revert --ignore-missing)
    echo "$output"
    
    if echo "$output" | grep -q "No migrations available to revert"; then
        echo "All migrations reverted."
        break
    fi
done

run_query "DROP TABLE IF EXISTS _sqlx_migrations"
run_query "DROP SCHEMA IF EXISTS apalis CASCADE"
