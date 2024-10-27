use anyhow::Result;
use recentf::database::upsert;
use sqlx::{query, PgPool};

pub fn mock_data() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("", vec!["/a/a", "/a/file", "/b/b", "/b/file"]),
        ("1", vec!["/a1/a1", "/a1/file", "/b1/b1", "/b1/file"]),
        ("2", vec!["/a2/a2", "/a2/file", "/b2/b2", "/b2/file"]),
        ("3", vec!["/a3/a3", "/a3/file", "/b3/b3", "/b3/file"]),
    ]
}

pub async fn insert_mock(pool: &PgPool) -> () {
    batch_insert(pool, mock_data()).await;
}

pub async fn batch_insert(pool: &PgPool, inserts: Vec<(&str, Vec<&str>)>) -> () {
    for (prefix, paths) in inserts {
        for path in paths {
            upsert(pool, &prefix, &path).await.unwrap();
        }
    }
}

pub async fn verify_tramp_id(conn: &PgPool, tramp_prefix: &str, expect: i16) -> Result<()> {
    let id = query!(
        r#"
SELECT id
FROM tramp
WHERE tramp_prefix = $1 
"#,
        tramp_prefix
    )
    .fetch_one(conn)
    .await
    .expect("could not fetch tramp id")
    .id;
    if id == expect {
        Ok(())
    } else {
        Err(anyhow::anyhow!("got tramp id: {}, expect: {}", id, expect))
    }
}
