pub mod get_all_flowers_dto;
use common::repository::flowers::FlowerRepository;

pub async fn run<T: FlowerRepository>(
    repo: &T,
    _req: get_all_flowers_dto::GetAllFlowersRequestDto,
) -> Result<get_all_flowers_dto::GetAllFlowersResponseDto, common::errors::AppError> {
    let flowers = repo.get_all_flowers().await?;
    Ok(get_all_flowers_dto::GetAllFlowersResponseDto { flowers })
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_type::models::flowers::create_model_flower_from_name;
    use common::errors::AppError;

    struct MockFlowerRepository;

    impl FlowerRepository for MockFlowerRepository {
        async fn get_all_flowers(&self) -> Result<Vec<common_type::models::flowers::Flower>, AppError> {
            Ok(vec![
                create_model_flower_from_name("Renge"),
                create_model_flower_from_name("Acacia"),
            ])
        }
        async fn insert_flower(&self, _flower: &common_type::models::flowers::Flower) -> Result<(), AppError> {
            Ok(())
        }
        async fn has_flower(&self, _flower: &common_type::models::flowers::Flower) -> Result<bool, AppError> {
            Ok(true)
        }
        async fn get_flower_id_by_name(&self, _name: &str) -> Option<i32> {
            Some(1)
        }
    }

    #[tokio::test]
    async fn test_run() {
        let repo = MockFlowerRepository;
        let req = get_all_flowers_dto::GetAllFlowersRequestDto {};
        let result = run(&repo, req).await.unwrap();

        assert_eq!(result.flowers.len(), 2);
        assert_eq!(result.flowers[0].name_jp, "Renge");
        assert_eq!(result.flowers[1].name_jp, "Acacia");
    }
}
