use sqlx::pool::PoolOptions;

#[tokio::main]
pub async fn main() {
    dotenvy::dotenv().ok();

    let pool = PoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("missing `DATABASE_URL` env variable"))
        .await
        .unwrap();

    api::run(
        pool,
        std::env::var("API_PORT")
            .expect("missing `API_PORT` env variable")
            .parse::<u16>()
            .expect("API_PORT must be a valid u16"),
    )
    .await;
}
