#!/bin/bash
#
# Runs the website and api binaries and watches them for changes

# Read the .env file
source .env

# Check if DEV_MODE is true
if [ "$PUBLIC_DEV_MODE" != "true" ]; then
    echo "Error: DEV_MODE is not set to true"
    exit 1
fi

# Function to run the API
run_api() {
    (cargo watch -x "run --package api --target-dir=target/api" 2>&1) | sed 's/^/[API] /'
}

# Function to run the job runner
run_job_runner() {
    (cargo watch -x "run --package jobs --target-dir=target/jobs" 2>&1) | sed 's/^/[JOBS] /'
}

# Function to run the website
run_website() {
    (cargo leptos watch 2>&1) | sed 's/^/[WEB] /'
}

# Run both processes in the background
run_api &
API_PID=$!

run_job_runner &
JOB_RUNNER_PID=$!

run_website &
WEBSITE_PID=$!

# Function to handle script termination
cleanup() {
    echo "Stopping processes..."
    pkill -P $API_PID
    pkill -P $JOB_RUNNER_PID
    pkill -P $WEBSITE_PID
    exit 0
}

# Set up trap to call cleanup function on script termination
trap cleanup SIGINT SIGTERM

# Wait for both processes to finish
wait $API_PID $WEBSITE_PID
