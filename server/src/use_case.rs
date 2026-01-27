pub mod get_all_beekeepers;
pub mod get_all_flowers;
pub mod get_all_honies;
pub mod get_all_prefectures;
pub mod put_new_honey;
pub mod put_new_honey_use_case;

use async_trait::async_trait;

#[async_trait]
pub trait UseCase<R, Req, Res> {
    async fn run(&self, repo: &R, req: Req) -> Res;
}