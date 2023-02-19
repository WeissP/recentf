use crate::search::{Candidate, Candidates, Query, Status};
use anyhow::Result;
use itertools::Itertools;
use sqlx::{query, PgPool};

macro_rules! new_candidate {
    ($record:expr) => {
        Candidate::new(
            $record.fullpath,
            $record.last_ref,
            $record.freq,
            $record.status,
        )
    };
}

pub async fn search(conn: &PgPool, q: Query) -> Result<Candidates> {
    let paths = wrap_like(q.paths);
    let names = wrap_like(q.names);
    let aliases = wrap_like(q.tramp_aliases);
    if aliases.is_empty() {
        search_without_tramp(&conn, &paths, &names).await
    } else {
        search_with_tramp(&conn, &aliases, &paths, &names).await
    }
}

async fn search_without_tramp(
    conn: &PgPool,
    paths: &[String],
    names: &[String],
) -> Result<Candidates> {
    let records = query!(
        r#"
SELECT fullpath, last_ref, freq, status as "status: Status"
FROM file
WHERE tramp_id = 0
AND deleted = false
AND status >= 2
AND dirpath ILIKE all ($1::text[])
AND filename ILIKE all ($2::text[])
ORDER BY status
"#,
        paths,
        names,
    )
    .fetch_all(conn)
    .await?;
    let candidates = records.into_iter().map(|x| new_candidate!(x)).collect();
    Ok(Candidates::single(String::new(), candidates))
}

async fn search_with_tramp(
    conn: &PgPool,
    aliases: &[String],
    paths: &[String],
    names: &[String],
) -> Result<Candidates> {
    let records = query!(
        r#"
SELECT tramp_prefix, fullpath, last_ref, freq, status as "status: Status"
FROM file INNER JOIN tramp ON file.tramp_id = tramp.id
WHERE tramp_id != 0
AND deleted = false
AND status >= 2
AND dirpath ILIKE all ($1::text[])
AND filename ILIKE all ($2::text[])
AND tramp_prefix ILIKE all ($3::text[])
ORDER BY tramp.id, status
"#,
        paths,
        names,
        aliases,
    )
    .fetch_all(conn)
    .await?;

    let mut candidates = Candidates::default();
    for (prefix, group) in &records.into_iter().group_by(|e| e.tramp_prefix.to_string()) {
        candidates
            .insert(prefix, group.map(|x| new_candidate!(x)).collect())
            .expect("id conflits during search_with_tramp");
    }
    Ok(candidates)
}

fn wrap_like<Seg: AsRef<str>>(segs: Vec<Seg>) -> Vec<String> {
    segs.iter().map(|x| format!("%{}%", x.as_ref())).collect()
}
