use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug)]
pub struct DBEngine {
    pub pool: Pool<Postgres>,
}

impl DBEngine {
    pub async fn build_connection() -> Result<Self, sqlx::Error> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }
}