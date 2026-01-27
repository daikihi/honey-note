use common::errors::AppError;
use common::repository::prefectures::PrefectureRepository;
use common_type::models::prefectures::Prefecture;

pub async fn run<T: PrefectureRepository>(repo: &T) -> Result<Vec<Prefecture>, AppError> {
    match repo.get_all_prefectures().await {
        Ok(prefectures) => Ok(prefectures),
        Err(e) => {
            log::error!("Failed to fetch prefectures: {}", e);
            Err(e)
        }
    }
}
