#!/bin/bash
#
# Inserts test user data into the database

# Read the .env file
source .env

# Check if DEV_MODE is true
if [ "$PUBLIC_DEV_MODE" != "true" ]; then
    echo "Error: DEV_MODE is not set to true"
    exit 1
fi
