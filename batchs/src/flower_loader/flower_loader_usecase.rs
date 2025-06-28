use crate::{flower_loader_master_gateway::load_master_data, flower_loader_request::FlowerLoaderRequestDto};

pub fn run(dto: FlowerLoaderRequestDto) {
    let master_file_name = dto.master_file_name.as_str();
    let db_file_name = dto.db_file_name;

    let master_data: String = load_master_data(&master_file_name);

}