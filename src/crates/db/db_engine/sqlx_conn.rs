use dotenvy::dotenv;
use sqlx::postgres::{PgArguments, PgPoolOptions, PgRow};
use sqlx::query::{Query, QueryAs};
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

    async fn mutate_one<T, F>(&self, sql: &'static str, bind: F) -> Result<T, sqlx::Error>
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
        self.mutate_one(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
            |q| q.bind(name.to_owned()).bind(email.to_owned()),
        )
        .await
    }

    pub async fn update_user(
        &self,
        id: i64,
        name: &str,
        email: &str,
    ) -> Result<Users, sqlx::Error> {
        self.mutate_one(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email",
            |q| q.bind(name.to_owned()).bind(email.to_owned()).bind(id),
        )
        .await
    }

    async fn full_read<T, F>(&self, sql: &'static str, bind: F) -> Result<Vec<T>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
        F: FnOnce(
            QueryAs<'static, Postgres, T, PgArguments>,
        ) -> QueryAs<'static, Postgres, T, PgArguments>,
    {
        bind(query_as::<_, T>(sql)).fetch_all(&self.pool).await
    }

    async fn single_read<T, F>(&self, sql: &'static str, bind: F) -> Result<T, sqlx::Error>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
        F: FnOnce(
            QueryAs<'static, Postgres, T, PgArguments>,
        ) -> QueryAs<'static, Postgres, T, PgArguments>,
    {
        bind(query_as::<_, T>(sql)).fetch_one(&self.pool).await
    }

    pub async fn read_all(&self) -> Result<Vec<Users>, sqlx::Error> {
        self.full_read::<Users, _>("SELECT * FROM users", |q| q)
            .await
    }

    pub async fn read_by_id(&self, id: i64) -> Result<Users, sqlx::Error> {
        self.single_read::<Users, _>("SELECT * FROM users WHERE id = $1", |q| q.bind(id))
            .await
    }

    async fn delete<F>(&self, sql: &'static str, bind: F) -> Result<u64, sqlx::Error>
    where
        F: FnOnce(Query<'static, Postgres, PgArguments>) -> Query<'static, Postgres, PgArguments>,
    {
        let mut tx = self.pool.begin().await?;

        let result = bind(query(sql)).execute(&mut *tx).await;

        match result {
            Ok(done) => {
                tx.commit().await?;
                Ok(done.rows_affected())
            }
            Err(e) => {
                let _ = tx.rollback().await;
                Err(e)
            }
        }
    }

    pub async fn delete_by_id(&self, id: i64) -> Result<u64, sqlx::Error> {
        self.delete("DELETE FROM users WHERE id = $1", |q| q.bind(id))
            .await
    }
}
