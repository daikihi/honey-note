#[derive(Debug)]
pub struct FlowerLoaderRequestDto {
    pub master_file_name: String,
    pub db_file_name: String,
}

impl FlowerLoaderRequestDto {
    pub fn new(master_file_name: String, db_file_name: String) -> Self {
        Self {
            master_file_name,
            db_file_name,
        }
    }
}
