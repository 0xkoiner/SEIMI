use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, query, query_as, Pool, Postgres, Transaction};

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

    pub async fn insert_user(
        &self,
        name: &str,
        email: &str,
    ) -> Result<Users, sqlx::Error> {
        let mut tx = self.tx().await.expect("Failed to begin transaction");

        let insert_res = query_as::<_, Users>(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        )
        .bind(name)
        .bind(email)
        .fetch_one(&mut *tx)
        .await;

        if let Err(e) = insert_res {
            println!("Failed to insert user: {e}");
            let _ = tx.rollback().await;
            return Err(e);
        }

        let query_res = query_as::<_, Users>("SELECT id, name, email FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&mut *tx)
            .await;

        match query_res {
            Ok(user) => {
                println!("Queried user: {:#?}", &user);
                tx.commit().await?;
                Ok(user)
            }
            Err(e) => {
                println!("Failed to query user after insertion: {e}");
                let _ = tx.rollback().await;
                Err(e)
            }
        }

    }
}
