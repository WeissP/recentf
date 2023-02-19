mod common;

use anyhow::{Context, Result};
use common::{db, search};
use recentf_lib::{
    database::{self, upsert},
    search::Query,
};
use sqlx::{query, PgPool};

#[sqlx::test]
fn search_test(pool: PgPool) -> () {
    db::insert_mock(&pool).await;

    let res = search::search!(&pool, [], [], []);
    assert_eq!(search::paths_with_id(res, ""), db::mock_data()[0].1);

    let res = search::search!(&pool, [], ["a"], []);
    assert_eq!(search::paths_with_id(res, ""), ["/a/a", "/a/file"]);

    let res = search::search!(&pool, [], ["a"], ["file"]);
    assert_eq!(search::paths_with_id(res, ""), ["/a/file"]);

    let query = search::query!(["1"], ["a"], ["file"]);
    let res = database::search(&pool, query).await.unwrap();
    assert!(res.get("").is_none());
    assert_eq!(search::paths_with_id(res, "1"), ["/a1/file"]);
}

// we ignore it because it needs to wait a long time for testing last_ref
#[ignore]
#[sqlx::test]
async fn basic_test(pool: PgPool) -> sqlx::Result<()> {
    upsert_test_single(&pool, "", "/tmp", "file", 0).await;
    upsert_test_single(
        &pool,
        "ssh:weiss@192.168.8.31",
        "/lib",
        "syncthing@.service1",
        1,
    )
    .await;
    upsert_test_single(
        &pool,
        "ssh:weiss@192.168.8.31",
        "/lib",
        "syncthing@.service2",
        1,
    )
    .await;
    upsert_test_single(
        &pool,
        "ssh:weiss@192.168.8.31|sudo:root@192.168.8.31",
        "/lib/systemd/system",
        "syncthing@.service",
        2,
    )
    .await;
    Ok(())
}

async fn upsert_test_single(
    pool: &PgPool,
    tramp_prefix: &str,
    dir: &str,
    file_name: &str,
    expect_id: i16,
) -> () {
    let got_file_path = format!("{}/{}", dir, file_name);
    let err_env = || format!("for path {}", got_file_path);
    macro_rules! get_file {
            () => {
              query!(
                          r#"
              SELECT *
              FROM file
              WHERE tramp_id = $1 AND fullpath = $2 
              "#,
                expect_id, got_file_path
                    )
                    .fetch_one(pool)
                    .await
                    .expect("could not fetch file")
                };
        }
    upsert(&pool, tramp_prefix, &got_file_path).await.unwrap();
    db::verify_tramp_id(&pool, tramp_prefix, expect_id)
        .await
        .context(err_env())
        .unwrap();
    assert_eq!(get_file!().fullpath, got_file_path);
    assert_eq!(get_file!().filename, file_name);
    assert_eq!(get_file!().dirpath, dir);

    let last_ref1 = get_file!().last_ref;
    assert_eq!(get_file!().freq, 1);
    std::thread::sleep(std::time::Duration::from_secs(1));

    upsert(&pool, tramp_prefix, &got_file_path).await.unwrap();
    let last_ref2 = get_file!().last_ref;
    assert_eq!(get_file!().freq, 2);

    assert!(
        last_ref2 > last_ref1,
        "the second referenced date {} should greater that the first {} {}",
        last_ref2,
        last_ref1,
        err_env(),
    );
}
