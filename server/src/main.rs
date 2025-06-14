mod controllers;

use actix_web::{HttpServer, App};

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    // Initialize logging


    // Start the server on port 8080
    HttpServer::new(|| 
     App::new()
            .service(controllers::health_checking::health_check)
        )
        .bind(("127.0.0.1", 8080))?
    .run()
    .await
    
}
