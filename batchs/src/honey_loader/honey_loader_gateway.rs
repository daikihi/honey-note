use crate::honey_loader_models::JsonHoney;

pub fn run(file_name: &str) -> Result<Vec<JsonHoney>, String> {
    // ログ出力
    log::info!("honey_loader リクエスト: {:?}", file_name);

    let file_content = std::fs::read_to_string(file_name)
        .map_err(|e| format!("ファイルを読み込めませんでした: {}: {}", file_name, e))?;
    file_content
        .lines()
        .enumerate()
        .map(|(index, line)| {
            serde_json::from_str::<JsonHoney>(line).map_err(|e| {
                format!(
                    "JSON の読み込みに失敗しました: {}:{}: {}",
                    file_name,
                    index + 1,
                    e
                )
            })
        })
        .collect()
}
