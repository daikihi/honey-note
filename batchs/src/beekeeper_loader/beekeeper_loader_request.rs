pub struct BeekeeperLoaderRequestDto<'a> {
    pub file_name: &'a str,
    pub db_file_name: &'a str,
}

impl<'a> BeekeeperLoaderRequestDto<'a> {
    pub fn new(file_name: &'a str, db_file_name: &'a str) -> Self {
        BeekeeperLoaderRequestDto {
            file_name,
            db_file_name,
        }
    }
}
