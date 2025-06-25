pub struct FlowerLoaderRequestDto {
    pub master_file_name: String,
    pub db_file_name: String,
}

impl FlowerLoaderRequestDto {
    pub fn new(_master_file_name: String, _db_file_name: String) -> FlowerLoaderRequestDto {
        Self {
            master_file_name: _master_file_name,
            db_file_name: _db_file_name,
        }
    }
}
