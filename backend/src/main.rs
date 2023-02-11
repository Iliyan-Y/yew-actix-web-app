use actix_web::{web, App, HttpServer};
use routes::index::index_route_config;

mod controllers;
mod routes;

pub struct AppState {
  app_name: String,
  deployment_env: String,
}

fn state_config(cfg: &mut web::ServiceConfig) {
  cfg.app_data(web::Data::new(AppState {
    app_name: "Actix Test Api".to_string(),
    deployment_env: "dev".to_string(),
  }));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new().service(
      web::scope("/api/v1")
        .configure(state_config)
        .configure(index_route_config),
    )
  })
  .bind(("127.0.0.1", 3000))?
  .run()
  .await
}
