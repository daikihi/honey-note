use log::error;

pub fn load_master_data(file_name: &str) -> String {
    // ここでマスターデータを読み込む処理を実装
    // 例えば、ファイルからデータを読み込んで文字列として返す
    match std::fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(e) => {
            let msg = format!("Error reading file {}: {}", file_name, e);
            error!("{}", msg);
            String::new() // エラー時は空の文字列を返す
        }
    }

}