use crate::honey_loader_models::JsonHoney;

pub fn run(file_name: &str) -> Vec<JsonHoney> {
    // ログ出力
    log::info!("honey_loader リクエスト: {:?}", file_name);

    let file_content: String =
        std::fs::read_to_string(file_name).expect("ファイルを読み込めませんでした");
    let file_tcontents: Vec<String> = file_content
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let json_contents: Vec<JsonHoney> = file_tcontents
        .iter()
        .filter_map(|line| serde_json::from_str::<JsonHoney>(line).ok())
        .collect();
    json_contents
}
