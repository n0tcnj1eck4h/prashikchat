use std::error::Error;

use sqlx::{Pool, Postgres, migrate::MigrateError, postgres::PgPoolOptions};

pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn connect(url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = PgPoolOptions::new().max_connections(5).connect(url).await?;
        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
}
