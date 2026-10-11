use common::repository::honeys::{HoneyRepository, HoneyRepositorySqlite};
use common_type::models::honey_detail::HoneyDetail;
use common_type::models::honey_detail_basic::HoneyDetailBasic;
use common_type::models::honey_detail_types::{BeekeeperName, FlowerName, HoneyNameJp};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

/// 更新処理が使うテーブルを、接続が一つのメモリ DB に用意する。
async fn setup_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    for statement in [
        "CREATE TABLE honey (id INTEGER PRIMARY KEY, name_jp TEXT NOT NULL, name_en TEXT, \
         beekeeper_id INTEGER, user_id INTEGER NOT NULL, origin_country TEXT, origin_region TEXT, \
         harvest_year INTEGER, purchase_date TEXT, note TEXT)",
        "CREATE TABLE beekeeper (id INTEGER PRIMARY KEY, name_jp TEXT NOT NULL, \
         user_id INTEGER NOT NULL, UNIQUE(user_id, name_jp))",
        "CREATE TABLE flower (id INTEGER PRIMARY KEY, name_jp TEXT NOT NULL, name_en TEXT, \
         scientific_name TEXT, short_note TEXT, flower_type TEXT, image_path TEXT, \
         user_id INTEGER NOT NULL, note TEXT)",
        "CREATE TABLE honey_flower (honey_id INTEGER NOT NULL, flower_id INTEGER NOT NULL, \
         UNIQUE(honey_id, flower_id))",
    ] {
        sqlx::query(statement).execute(&pool).await.unwrap();
    }
    sqlx::query(
        "INSERT INTO honey (id, name_jp, name_en, user_id, note) \
         VALUES (10, '編集前', 'English name', 1, '残すメモ')",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO flower (id, name_jp, user_id) VALUES (100, '旧蜜源', 1)")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO honey_flower (honey_id, flower_id) VALUES (10, 100)")
        .execute(&pool)
        .await
        .unwrap();
    pool
}

/// 画面の入力に相当する蜂蜜詳細を作る。
fn edited_honey(beekeeper: Option<&str>, flowers: &[&str]) -> HoneyDetail {
    HoneyDetail {
        basic: HoneyDetailBasic {
            name_jp: HoneyNameJp("編集後".to_string()),
            beekeeper_name: beekeeper.map(|name| BeekeeperName(name.to_string())),
            harvest_year: None,
            country: None,
            region: None,
            flower_names: flowers
                .iter()
                .map(|name| FlowerName((*name).to_string()))
                .collect(),
            honey_type: None,
            volume: None,
            purchase_date: None,
        },
        dynamic: vec![],
        created_at: None,
        updated_at: None,
    }
}

/// 関連の置換・解除、所有者分離、画面にない項目の保持を確認する。
#[tokio::test]
async fn update_replaces_and_clears_relations_without_changing_unedited_fields() {
    let pool = setup_db().await;
    sqlx::query("INSERT INTO flower (id, name_jp, user_id) VALUES (200, 'レンゲ', 2)")
        .execute(&pool)
        .await
        .unwrap();
    let repository = HoneyRepositorySqlite { pool: pool.clone() };

    repository
        .update_honey(10, edited_honey(Some(" 養蜂場 "), &["レンゲ", "レンゲ"]), 1)
        .await
        .unwrap();

    let (name, english_name, note, beekeeper_id): (String, String, String, i32) =
        sqlx::query_as("SELECT name_jp, name_en, note, beekeeper_id FROM honey WHERE id = 10")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(name, "編集後");
    assert_eq!(english_name, "English name");
    assert_eq!(note, "残すメモ");
    let (beekeeper_name, beekeeper_owner): (String, i32) =
        sqlx::query_as("SELECT name_jp, user_id FROM beekeeper WHERE id = ?")
            .bind(beekeeper_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!((beekeeper_name.as_str(), beekeeper_owner), ("養蜂場", 1));
    let relations: Vec<(i32, String, i32)> = sqlx::query_as(
        "SELECT f.id, f.name_jp, f.user_id FROM honey_flower hf \
         JOIN flower f ON f.id = hf.flower_id WHERE hf.honey_id = 10",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(relations.len(), 1);
    assert_eq!(relations[0].1, "レンゲ");
    assert_eq!(relations[0].2, 1);
    assert_ne!(relations[0].0, 200);

    repository
        .update_honey(10, edited_honey(None, &[]), 1)
        .await
        .unwrap();
    let (beekeeper_id,): (Option<i32>,) =
        sqlx::query_as("SELECT beekeeper_id FROM honey WHERE id = 10")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(beekeeper_id, None);
    let (relation_count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM honey_flower WHERE honey_id = 10")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(relation_count, 0);
}

/// 蜜源の作成途中で失敗した場合、先行する書き込みを残さないことを確認する。
#[tokio::test]
async fn failed_flower_insert_rolls_back_honey_and_all_relations() {
    let pool = setup_db().await;
    sqlx::query(
        "CREATE TRIGGER fail_flower BEFORE INSERT ON flower \
         WHEN NEW.name_jp = '失敗' BEGIN SELECT RAISE(ABORT, 'テスト用エラー'); END",
    )
    .execute(&pool)
    .await
    .unwrap();
    let repository = HoneyRepositorySqlite { pool: pool.clone() };

    assert!(repository
        .update_honey(10, edited_honey(Some("新養蜂場"), &["成功", "失敗"]), 1)
        .await
        .is_err());

    let (name, beekeeper_id): (String, Option<i32>) =
        sqlx::query_as("SELECT name_jp, beekeeper_id FROM honey WHERE id = 10")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(name, "編集前");
    assert_eq!(beekeeper_id, None);
    let (beekeeper_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM beekeeper")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(beekeeper_count, 0);
    let (flower_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM flower")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(flower_count, 1);
    let (flower_id,): (i32,) =
        sqlx::query_as("SELECT flower_id FROM honey_flower WHERE honey_id = 10")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(flower_id, 100);
}

/// 蜂蜜本体と関連の更新後に失敗した場合、以前の状態へ戻ることを確認する。
#[tokio::test]
async fn failed_relation_insert_restores_the_previous_honey_and_flower() {
    let pool = setup_db().await;
    sqlx::query(
        "CREATE TRIGGER fail_relation BEFORE INSERT ON honey_flower \
         WHEN NEW.flower_id <> 100 BEGIN SELECT RAISE(ABORT, 'テスト用エラー'); END",
    )
    .execute(&pool)
    .await
    .unwrap();
    let repository = HoneyRepositorySqlite { pool: pool.clone() };

    assert!(repository
        .update_honey(10, edited_honey(Some("新養蜂場"), &["新蜜源"]), 1)
        .await
        .is_err());

    let (name, beekeeper_id, english_name, note): (String, Option<i32>, String, String) =
        sqlx::query_as("SELECT name_jp, beekeeper_id, name_en, note FROM honey WHERE id = 10")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(name, "編集前");
    assert_eq!(beekeeper_id, None);
    assert_eq!(english_name, "English name");
    assert_eq!(note, "残すメモ");
    let (flower_id,): (i32,) =
        sqlx::query_as("SELECT flower_id FROM honey_flower WHERE honey_id = 10")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(flower_id, 100);
    let (flower_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM flower")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(flower_count, 1);
    let (beekeeper_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM beekeeper")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(beekeeper_count, 0);
}
