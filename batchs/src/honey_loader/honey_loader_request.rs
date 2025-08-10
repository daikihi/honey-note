#[derive(Debug)]
pub struct HoneyLoaderRequestDto<'a> {
    pub file_name: &'a str,
    pub db_file_name: &'a str,
}

impl<'a> HoneyLoaderRequestDto<'a> {
    pub fn new(file_name: &'a str, db_file_name: &'a str) -> Self {
        HoneyLoaderRequestDto {
            file_name,
            db_file_name,
        }
    }
}
