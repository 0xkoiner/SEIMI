use dotenvy::dotenv;
use sqlx::postgres::{PgArguments, PgPoolOptions, PgRow};
use sqlx::query::QueryAs;
use sqlx::{FromRow, Pool, Postgres, Transaction, query, query_as};

use crate::db::types::schema::Users;

#[derive(Debug)]
pub struct DBEngine {
    pub pool: Pool<Postgres>,
}

impl DBEngine {
    pub async fn build_connection() -> Result<Self, sqlx::Error> {
        dotenv().ok();
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn tx(&self) -> Result<Transaction<'_, Postgres>, sqlx::Error> {
        self.pool.begin().await
    }

    pub async fn init_db(&self) -> Result<(), sqlx::Error> {
        let mut tx = self.tx().await.expect("Failed to begin transaction");

        query(
            "CREATE TABLE IF NOT EXISTS users (
                id BIGSERIAL PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE
            )",
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn insert<T, F>(&self, sql: &'static str, bind: F) -> Result<T, sqlx::Error>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
        F: FnOnce(
            QueryAs<'static, Postgres, T, PgArguments>,
        ) -> QueryAs<'static, Postgres, T, PgArguments>,
    {
        let mut tx = self.pool.begin().await?;

        let result = bind(query_as::<_, T>(sql)).fetch_one(&mut *tx).await;

        match result {
            Ok(row) => {
                tx.commit().await?;
                Ok(row)
            }
            Err(e) => {
                let _ = tx.rollback().await;
                Err(e)
            }
        }
    }

    pub async fn insert_user(&self, name: &str, email: &str) -> Result<Users, sqlx::Error> {
        self.insert(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
            |q| q.bind(name.to_owned()).bind(email.to_owned()),
        )
        .await
    }

    async fn read<T>(&self, sql: &'static str) -> Result<Vec<T>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
    {
        query_as::<_, T>(sql).fetch_all(&self.pool).await
    }

    pub async fn read_all(&self) -> Result<Vec<Users>, sqlx::Error> {
        self.read::<Users>("SELECT * FROM users").await
    }
}
