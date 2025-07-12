pub struct beekeeper_loader_request_dto<'a> {
    pub file_name: &'a str,
    pub db_file_name: &'a str,
}

impl<'a> beekeeper_loader_request_dto<'a> {
    pub fn new(file_name: &'a str, db_file_name: &'a str) -> Self {
        beekeeper_loader_request_dto {
            file_name,
            db_file_name,
        }
    }
}
