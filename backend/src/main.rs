use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use routes::index::index_route_config;
use sea_orm::{Database, DatabaseConnection};

mod controllers;
mod entity;
mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
  app_name: String,
  db: DatabaseConnection,
}

async fn make_db_pool() -> DatabaseConnection {
  let db_url = std::env::var("DATABASE_URL").unwrap();
  let db = Database::connect(db_url).await.unwrap();
  return db;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  pretty_env_logger::init();
  dotenv::dotenv().ok();

  let state = AppState {
    app_name: "Actix Test Api".to_string(),
    db: make_db_pool().await,
  };

  HttpServer::new(move || {
    // EXAMPLE CORS PRODUCTION
    let cors = Cors::default()
      .allowed_origin("http://127.0.0.1:8080")
      .allowed_methods(vec!["GET", "POST"])
      .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
      .allowed_header(http::header::CONTENT_TYPE)
      .max_age(3600);

    // For DEV env only !
    // let cors = Cors::permissive();

    App::new().wrap(cors).service(
      web::scope("/api/v1")
        .app_data(web::Data::new(state.clone()))
        .configure(index_route_config), //  .wrap(middleware::Logger::default()),
    )
  })
  .bind(("127.0.0.1", 3000))?
  .run()
  .await?;

  Ok(())
}
