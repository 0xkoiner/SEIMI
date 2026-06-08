use std::time::Duration;

use dotenvy::dotenv;
use sqlx::postgres::{PgArguments, PgPoolOptions, PgRow};
use sqlx::query::{Query, QueryAs};
use sqlx::{FromRow, Pool, Postgres, Transaction, query, query_as};
use tracing::{info, instrument};

use crate::db::types::errors::DbError;
use crate::db::types::schema::Users;

#[derive(Debug)]
pub struct DBEngine {
    pub pool: Pool<Postgres>,
}

impl DBEngine {
    pub async fn build_connection() -> Result<Self, DbError> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| DbError::Config("DATABASE_URL not set".into()))?;

        let pool = PgPoolOptions::new()
            .max_connections(20)
            .min_connections(2)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Some(Duration::from_secs(600)))
            .max_lifetime(Some(Duration::from_secs(1800)))
            .test_before_acquire(true)
            .after_connect(|conn, _meta| {
                Box::pin(async move {
                    sqlx::query("SET statement_timeout = '30s'")
                        .execute(&mut *conn)
                        .await
                        .map(|_| ())
                })
            })
            .connect(&database_url)
            .await?;

        info!("DB pool connected");
        Ok(Self { pool })
    }

    pub async fn tx(&self) -> Result<Transaction<'_, Postgres>, DbError> {
        Ok(self.pool.begin().await?)
    }

    #[instrument(skip(self))]
    pub async fn init_db(&self) -> Result<(), DbError> {
        sqlx::migrate!("src/crates/db/migrations")
            .run(&self.pool)
            .await?;
        info!("Migrations applied");
        Ok(())
    }

    #[instrument(skip_all)]
    pub(crate) async fn mutate_one<T, F>(&self, sql: &'static str, bind: F) -> Result<T, DbError>
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
                Err(e.into())
            }
        }
    }

    pub async fn insert_user(&self, name: &str, email: &str) -> Result<Users, DbError> {
        self.mutate_one(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
            |q| q.bind(name.to_owned()).bind(email.to_owned()),
        )
        .await
    }

    pub async fn update_user(&self, id: i64, name: &str, email: &str) -> Result<Users, DbError> {
        self.mutate_one(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email",
            |q| q.bind(name.to_owned()).bind(email.to_owned()).bind(id),
        )
        .await
    }

    #[instrument(skip_all)]
    pub(crate) async fn full_read<T, F>(
        &self,
        sql: &'static str,
        bind: F,
    ) -> Result<Vec<T>, DbError>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
        F: FnOnce(
            QueryAs<'static, Postgres, T, PgArguments>,
        ) -> QueryAs<'static, Postgres, T, PgArguments>,
    {
        bind(query_as::<_, T>(sql))
            .fetch_all(&self.pool)
            .await
            .map_err(Into::into)
    }

    #[instrument(skip_all)]
    pub(crate) async fn single_read<T, F>(&self, sql: &'static str, bind: F) -> Result<T, DbError>
    where
        T: for<'r> FromRow<'r, PgRow> + Send + Unpin,
        F: FnOnce(
            QueryAs<'static, Postgres, T, PgArguments>,
        ) -> QueryAs<'static, Postgres, T, PgArguments>,
    {
        bind(query_as::<_, T>(sql))
            .fetch_one(&self.pool)
            .await
            .map_err(Into::into)
    }

    pub async fn read_all(&self) -> Result<Vec<Users>, DbError> {
        self.full_read::<Users, _>("SELECT * FROM users", |q| q)
            .await
    }

    pub async fn read_by_id(&self, id: i64) -> Result<Users, DbError> {
        self.single_read::<Users, _>("SELECT * FROM users WHERE id = $1", |q| q.bind(id))
            .await
    }

    #[instrument(skip_all)]
    pub(crate) async fn delete<F>(&self, sql: &'static str, bind: F) -> Result<u64, DbError>
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
                Err(e.into())
            }
        }
    }

    pub async fn delete_by_id(&self, id: i64) -> Result<u64, DbError> {
        self.delete("DELETE FROM users WHERE id = $1", |q| q.bind(id))
            .await
    }
}
