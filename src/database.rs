use anyhow::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::sync::Arc;

pub struct Database {
    pub pool: Arc<PgPool>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            // Default to local PostgreSQL if not set
            "postgresql://postgres:postgres@localhost:5432/aichat".to_string()
        });

        let pool = PgPoolOptions::new()
            .max_connections(25)
            .connect(&database_url)
            .await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub async fn run_migrations(&self) -> Result<()> {
        // In production, you'd use sqlx migrate! macro
        // For now, we'll check if tables exist and create them if not
        let migration_sql = include_str!("../migrations/001_initial_schema.sql");

        // Split by statement and execute each
        for statement in migration_sql.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                let _ = sqlx::query(trimmed)
                    .execute(&*self.pool)
                    .await
                    .map_err(|e| {
                        // Ignore "already exists" errors
                        if e.to_string().contains("already exists") {
                            log::debug!("Schema object already exists, skipping: {}", e);
                        } else {
                            log::error!("Migration error: {}", e);
                        }
                    });
            }
        }

        log::info!("Database migrations completed");
        Ok(())
    }

    pub fn pool(&self) -> Arc<PgPool> {
        self.pool.clone()
    }
}