#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("row not found")]
    NotFound,
    #[error("unique constraint violated: {constraint}")]
    UniqueViolation { constraint: String },
    #[error("foreign key violated: {constraint}")]
    FkViolation { constraint: String },
    #[error("missing config: {0}")]
    Config(String),
    #[error("migration error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error("{0}")]
    Sqlx(sqlx::Error),
}

impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        match &e {
            sqlx::Error::RowNotFound => DbError::NotFound,
            sqlx::Error::Database(db) => {
                let code = db.code().as_deref().map(str::to_owned);
                let constraint = db.constraint().unwrap_or("?").to_string();
                match code.as_deref() {
                    Some("23505") => DbError::UniqueViolation { constraint },
                    Some("23503") => DbError::FkViolation { constraint },
                    _ => DbError::Sqlx(e),
                }
            }
            _ => DbError::Sqlx(e),
        }
    }
}
