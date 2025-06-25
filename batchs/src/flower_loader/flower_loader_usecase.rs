pub mod usecase {
    use crate::flower_loader_request::FlowerLoaderRequestDto;
    use crate::resources_master_gateway;

    pub fn run(dto: FlowerLoaderRequestDto) {
        let _master_file_name = dto.master_file_name;
        let file_contents = resources_master_gateway::gateway::get_flower_list(&_master_file_name);

        file_contents.lines().for_each(|line| {
            println!("{}", line);
        });
    }
}
