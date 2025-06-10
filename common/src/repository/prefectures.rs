use crate::models::prefectures::Prefecture as PrefectureModel;
use crate::infrastructure::db::sqlx::prefecture;
use log::info;

pub async  fn has_prefecture(_model_prefecture: &PrefectureModel, pool: &sqlx::SqlitePool) -> Result<bool, sqlx::Error> {
    let db_prefecture: prefecture::Prefecture = prefecture::Prefecture::from_model(&PrefectureModel {
        id: 0, // ID is not used in this check
        name_jp: _model_prefecture.name_jp.clone(),
        name_en: _model_prefecture.name_en.clone(), // English name is not used in this check
    });

    for_logging(&db_prefecture, "Checking if prefecture exists in database");

    let result =  db_prefecture.has_prefecture(pool)
        .await;
    // @todo should not return sqlx::Error to application layer
    result
}

pub async fn insert_prefecture(_model_prefecture: &PrefectureModel, pool: &sqlx::SqlitePool) { 
    let db_prefecture: prefecture::Prefecture = prefecture::Prefecture::from_model(_model_prefecture);
    let _cloned = db_prefecture.clone();
    for_logging(&db_prefecture, "Inserting prefecture into database");

    db_prefecture.insert_prefecture(pool)
        .await
        .expect("Failed to insert prefecture");
}

fn for_logging(db_prefecture: &prefecture::Prefecture, msg: &str) { 
    info!("{}, {:?}", msg, db_prefecture);
}