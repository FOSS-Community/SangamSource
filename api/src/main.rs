use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use std::env;

mod auth;
mod db;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = db::init(database_url).await.expect("Failed to initialize database");

    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    println!("Starting server at: {}", &server_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(db_pool.clone())
            .configure(auth::init_config)
            .configure(users::init_config)
            .route("/", web::get().to(greet))
    })
    .bind(&server_address)?
    .run()
    .await
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Sangam API")
}
