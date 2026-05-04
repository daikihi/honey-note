use crate::honey_loader_request::HoneyLoaderRequestDto;

mod honey_loader_gateway;
mod honey_loader_models;
mod honey_loader_request;
mod honey_loader_usecase;

/**
 * ハチミツデータをデータベースに保存するバッチプロセス
 *
 * 目的:
 *   指定されたファイルからハチミツ情報を読み込み、指定ユーザーのデータとしてSQLiteデータベースに登録する
 *
 * 引数:
 *   args[1]: ハチミツデータファイルのパス（通常は CSV または JSONL 形式）
 *   args[2]: SQLiteデータベースのファイルパス
 *   args[3]: データを登録するユーザーID（数値）
 *
 * 特徴:
 *   - 各ユーザーのハチミツデータは独立して管理される
 *   - ユーザーIDに基づいてデータをフィルタリング・関連付けを行う
 *   - バッチ実行ごとに指定ユーザーのデータを更新・追加する
 */
#[tokio::main]
async fn main() {
    println!("honey_loader 起動");
    env_logger::init();
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    let file_name: &String = args.get(1).expect("ファイル名を指定してください");
    let db_file_name: &String = args
        .get(2)
        .expect("データベースのファイル名を指定してください");
    let user_id = args
        .get(3)
        .expect("ユーザーIDを指定してください")
        .parse::<i32>()
        .expect("ユーザーIDは数値で指定してください");
    let request_dto: HoneyLoaderRequestDto = HoneyLoaderRequestDto::new(file_name, db_file_name);
    let _use_case_result = honey_loader_usecase::run(request_dto, user_id).await;
}
