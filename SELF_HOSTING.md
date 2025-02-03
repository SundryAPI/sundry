# Sundry

Sundry is open source and can be self hosted.

## Quickstart

Docker compose will get everything running locally, and even watch for changes, but you'll want to run services outside compose with cargo watch to support incremental development.

    $ docker compose up

Or, for the brave:
    
    $ docker compose watch

## Running services locally

You can run the services individually, e.g.

    $ docker compose up postgres -d
    $ docker compose up migrations -d
    $ docker compose up website -d
    $ docker compose up api -d
    $ docker compose up jobs -d

TODO stabilize and move all of this into a proper dev setup script. See the docker-compose.yml for details.

### Install Rust and command line utils

    $ rustup default 1.83
    $ rustup target add wasm32-unknown-unknown
    $ cargo install cargo-pgrx --version=0.11.3
    $ cargo install cargo-leptos
    $ cargo install sqlx-cli

### Database setup

You can use any local or self hosted postgres database that supports transactions.

### ENV setup

    $ cp .env.development .env

Be sure and set all env variables.

### Run database migrations

    $ sqlx migrate run

### Run tests
    
    $ cargo test

### Run the website

    $ cargo leptos watch

### Run the API

    $ cargo run --bin api 

### Run the workers for jobs

    $ cargo run --bin job_runner

### Test the api dockerfile

    $ docker build -f Dockerfile.api -t 'api' .
    $ docker run 

