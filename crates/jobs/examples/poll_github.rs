use anyhow::Result;
use apalis::postgres::{PgPool, PostgresStorage};

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug,sqlx::query=error");
    tracing_subscriber::fmt::init();
    let database_url = std::env::var("DATABASE_URL").expect("Must specify path to db");

    let pool = PgPool::connect(&database_url).await?;
    PostgresStorage::migrations()
        .set_ignore_missing(true)
        .run(&pool)
        .await
        .expect("unable to run migrations for postgres");

    // let mut pg = PostgresStorage::new(pool);

    // pg.push(Ingest { organization_id: 1 }).await?;

    Ok(())
}
