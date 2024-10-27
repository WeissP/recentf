use anyhow::Result;
use sqlx::{query, PgPool};

pub async fn upsert(conn: &PgPool, prefix: &str) -> Result<i16> {
    let id = match try_find(conn, prefix).await? {
        Some(id) => id,
        None => {
            query!(
                r#"
INSERT INTO tramp (tramp_prefix)
VALUES ($1) RETURNING id;
"#,
                prefix
            )
            .fetch_one(conn)
            .await?
            .id
        }
    };
    Ok(id)
}

pub async fn try_find(conn: &PgPool, prefix: &str) -> Result<Option<i16>> {
    let res = query!(
        r#"
SELECT id
FROM tramp
WHERE tramp_prefix = $1
"#,
        prefix
    )
    .fetch_optional(conn)
    .await?;
    Ok(res.map(|x| x.id))
}
