use super::tramp::Tramp;
use anyhow::{Context, Result};
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};

#[derive(PartialEq, Eq)]
pub enum Status {
    Deleted,
    Normal,
    Favourite,
}
pub struct Unix(i64);
pub struct File {
    pub tramp_id: i32,
    pub fullpath: String,
    pub dirpath: String,
    pub filename: String,
    pub last_ref: Unix,
    pub freq: i32,
    pub status: Status,
}

pub struct FileKey<'a> {
    tramp_id: i32,
    full_path: &'a Path,
}

pub struct FileConn<'a> {
    key: FileKey<'a>,
    pool: &'a SqlitePool,
}

impl<'a> FileConn<'a> {
    async fn matched_file(&self) -> Result<Option<File>> {
        todo!()
    }

    async fn matched_tramp(&self) -> Result<Option<Tramp>> {
        Tramp::with_id(self.pool, self.key.tramp_id).await
    }

    pub async fn update_priority(&self) -> Result<()> {
        todo!()
    }

    pub async fn new_access(&self) -> Result<()> {
        todo!()
    }

    /// return false if the path is in black list
    pub async fn insert(&self) -> Result<bool> {
        todo!()
    }

    /// Update or insert `fullpath` with given `tramp_id`, if `tramp_id` does not exist, insert new one
    pub async fn upsert(&self) -> Result<()> {
        match self.matched_file().await? {
            Some(f) if f.status == Status::Favourite => {
                // update unix and freq
                todo!()
            }
            Some(_) => {
                // Status is either deleted or norml, update to normal anyway
                todo!()
            }
            None => self.insert().await?,
        };

        Ok(())
    }
}
