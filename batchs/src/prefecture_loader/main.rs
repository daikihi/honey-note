extern crate log;
mod prefecture_loader_request;
mod prefecture_usecase;

use log::info;
use std::env;

use common::infrastructure::db::sqlx::get_sqlite_pool;
use prefecture_loader_request::PrefectureLoaderRequestDto;

/**
 * 日本の都道府県マスタデータをデータベースに保存するバッチプロセス
 *
 * 目的:
 *   指定されたCSVファイルから都道府県情報を読み込み、SQLiteデータベースに登録する
 *
 * 引数:
 *   args[1]: マスタデータファイルのパス（通常は CSV 形式）
 *   args[2]: SQLiteデータベースのファイルパス
 *
 * 特徴:
 *   - 都道府県は全ユーザーで共通の静的マスタデータのため、UserID は考慮しない
 *   - 各実行は独立した処理として動作する（バッチ実行ごとに再初期化可能）
 */

#[tokio::main]
async fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    info!("args: {:?}", args);

    if args.len() != 3 {
        panic!("Usage: prefecture_loader <file_name>");
    }

    let file_name = &args[1];
    let db_file_name = &args[2];
    info!("prefecture_loader is starting to run ...{:?}", file_name);

    let pool = get_sqlite_pool(db_file_name.to_string());
    let _dao: PrefectureLoaderRequestDto = PrefectureLoaderRequestDto {
        file_name: file_name.to_string(),
        pool: &pool,
    };

    let _ = prefecture_usecase::run(_dao).await;

    info!("complete prefecture_loader");
}
