use anyhow::{Context, Result};
use sqlx::SqlitePool;

pub struct Tramp {
    pub id: i32,
    pub alias: String,
    pub tramp_type: String,
    pub tramp_path: String,
}

impl Tramp {
    /// insert a new tramp entity, return the entity id
    pub async fn insert(
        pool: &SqlitePool,
        alias: Option<&str>,
        tramp_type: &str,
        tramp_path: &str,
    ) -> Result<i32> {
        todo!()
    }

    pub async fn with_id(pool: &SqlitePool, id: i32) -> Result<Option<Self>> {
        todo!()
    }

    pub async fn with_alias(pool: &SqlitePool, id: i32) -> Result<Vec<Self>> {
        todo!()
    }

    pub async fn with_type_path(
        pool: &SqlitePool,
        tramp_type: &str,
        tramp_path: &str,
    ) -> Result<Vec<Self>> {
        todo!()
    }

    pub async fn remove(pool: &SqlitePool, id: i32) -> Result<()> {
        todo!()
    }
}
