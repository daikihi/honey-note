pub mod get_all_flowers_dto;
use common::repository::flowers::FlowerRepository;

pub async fn run<T: FlowerRepository>(
    repo: &T,
    _req: get_all_flowers_dto::GetAllFlowersRequestDto,
    user_id: i32,
) -> Result<get_all_flowers_dto::GetAllFlowersResponseDto, common::errors::AppError> {
    let flowers = repo.get_all_flowers(user_id).await?;
    Ok(get_all_flowers_dto::GetAllFlowersResponseDto { flowers })
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_type::models::flowers::create_model_flower_from_name;
    use common::errors::AppError;

    struct MockFlowerRepository;

    #[async_trait::async_trait]
    impl FlowerRepository for MockFlowerRepository {
        async fn get_all_flowers(&self, _user_id: i32) -> Result<Vec<common_type::models::flowers::Flower>, AppError> {
            Ok(vec![
                create_model_flower_from_name("Renge"),
                create_model_flower_from_name("Acacia"),
            ])
        }
        async fn get_flower_by_id(&self, _id: i32, _user_id: i32) -> Result<common_type::models::flowers::Flower, AppError> {
            Ok(create_model_flower_from_name("Mock Flower"))
        }
        async fn update_flower<'a, E>(
            &self,
            _id: i32,
            _flower: &common_type::models::flowers::Flower,
            _user_id: i32,
            _executor: E,
        ) -> Result<(), AppError>
        where
            E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
        {
            Ok(())
        }
        async fn exists_flower_by_id<'a, E>(&self, _id: i32, _user_id: i32, _executor: E) -> Result<bool, AppError>
        where
            E: sqlx::Executor<'a, Database = sqlx::Sqlite>,
        {
            Ok(true)
        }
        async fn insert_flower<'a, E>(&self, _flower: &common_type::models::flowers::Flower, _user_id: i32, _executor: E) -> Result<(), AppError>
        where
            E: sqlx::Executor<'a, Database = sqlx::Sqlite>
        {
            Ok(())
        }
        async fn has_flower<'a, E>(&self, _flower: &common_type::models::flowers::Flower, _user_id: i32, _executor: E) -> Result<bool, AppError>
        where
            E: sqlx::Executor<'a, Database = sqlx::Sqlite>
        {
            Ok(true)
        }
        async fn get_flower_id_by_name<'a, E>(&self, _name: &str, _user_id: i32, _executor: E) -> Option<i32>
        where
            E: sqlx::Executor<'a, Database = sqlx::Sqlite>
        {
            Some(1)
        }
    }

    #[tokio::test]
    async fn test_run() {
        let repo = MockFlowerRepository;
        let req = get_all_flowers_dto::GetAllFlowersRequestDto {};
        let result = run(&repo, req, 1).await.unwrap();

        assert_eq!(result.flowers.len(), 2);
        assert_eq!(result.flowers[0].name_jp, "Renge");
        assert_eq!(result.flowers[1].name_jp, "Acacia");
    }
}
