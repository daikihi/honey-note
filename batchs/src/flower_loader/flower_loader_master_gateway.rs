use log::error;

pub fn load_master_data(file_name: &str) -> String {
    match std::fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(e) => {
            let msg = format!("Error reading file {}: {}", file_name, e);
            error!("{}", msg);
            String::new() // エラー時は空の文字列を返す
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_load_master_data() {
        let test_file = "../resources/master_data/tests/test_data_flower.csv";
        let content = load_master_data(test_file);
        assert!(!content.is_empty(), "Failed to load master data");
        assert_eq!(
            content.lines().count(),
            3,
            "Unexpected number of lines in master data"
        );
    }
}
