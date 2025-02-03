FROM rust:1.83-bullseye

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends \
    openssl \
    ca-certificates \
    pkg-config \
    postgresql-client \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /app
COPY migrations ./migrations

CMD echo "Database URL (masked):" && \
    echo ${DATABASE_URL} | sed 's/:[^:]*@/:****@/' && \
    echo "Checking applied migrations:" && \
    (psql ${DATABASE_URL} -c "SELECT * FROM _sqlx_migrations ORDER BY version;" || echo "no migrations") && \
    echo "Local migration files:" && \
    ls -la migrations/ && \
    sqlx migrate run --database-url=${DATABASE_URL} --ignore-missing
