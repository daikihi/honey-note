pub mod gateway {
    use std::fs;

    pub fn get_flower_list(_master_file_name: &String) -> String {
        match fs::read_to_string(_master_file_name) {
            Ok(content) => content,
            Err(e) => panic!("{:?}", e),
        }
    }
}
