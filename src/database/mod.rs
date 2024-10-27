mod search;
mod tramp_db;

use crate::search::Status;
use anyhow::{Context, Result};
pub use search::search;
use sqlx::{migrate::Migrator, query, PgPool};
use std::path::Path;

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn migrate(conn: &PgPool) -> Result<()> {
    MIGRATOR.run(conn).await.context("could not migrate")
}

pub async fn connect(db_path: &str) -> Result<PgPool> {
    PgPool::connect(db_path)
        .await
        .context("could not connect to database")
}

pub async fn upsert<'a, 'b>(
    conn: &PgPool,
    tramp_prefix: &'b str,
    file_path: &'b str,
) -> Result<(i16, &'b str)> {
    let id = tramp_db::upsert(conn, tramp_prefix).await?;
    let p = Path::new(file_path);
    let file_name = p
        .file_name()
        .context("path must contain file name")?
        .to_str()
        .context("invalid path")?;
    let dir = p
        .parent()
        .context("path must contain parent dir")?
        .to_str()
        .context("invalid path")?;
    let full_path = p.to_str().context("invalid path")?;
    query!(
        r#"
INSERT INTO file(tramp_id,fullpath,dirpath,filename,    last_ref              , freq, deleted, status) VALUES
                ($1      ,      $2,     $3,      $4, extract(epoch from now()),    1, false  , 2 )
ON CONFLICT(tramp_id,fullpath)
DO UPDATE SET last_ref=EXCLUDED.last_ref, freq=file.freq+1, deleted=false, status=file.status
"#,
        id,
        full_path,
        dir,
        file_name
     ).execute(conn).await?;
    Ok((id, file_path))
}

pub async fn change_status(
    conn: &PgPool,
    tramp_prefix: &str,
    file_path: &str,
    status: Status,
) -> Result<()> {
    let (tramp_id, _) = upsert(conn, tramp_prefix, file_path).await?;
    query!(
        r#"
UPDATE file SET status = $1
 WHERE tramp_id = $2 AND fullpath = $3
"#,
        status as i32,
        tramp_id,
        file_path
    )
    .execute(conn)
    .await
    .context("could not update status")?;
    Ok(())
}

pub async fn change_deleted_flag(
    conn: &PgPool,
    tramp_prefix: &str,
    file_path: &str,
    deleted: bool,
) -> Result<()> {
    if let Some(tramp_id) = tramp_db::try_find(conn, tramp_prefix).await? {
        query!(
            r#"
UPDATE file SET deleted = $1
 WHERE tramp_id = $2 AND fullpath like $3
"#,
            deleted,
            tramp_id,
            format!("{}%", file_path)
        )
        .execute(conn)
        .await
        .context("could not set file as deleted")?;
    }
    Ok(())
}
