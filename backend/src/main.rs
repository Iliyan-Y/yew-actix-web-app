use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod routes;

struct AppState {
  app_name: String,
  deployment_env: String,
}

async fn manual_hello(data: web::Data<AppState>) -> impl Responder {
  let res = format!("HELOO from {}, {}", &data.app_name, &data.deployment_env);
  HttpResponse::Ok().body(res)
}

fn state_config(cfg: &mut web::ServiceConfig) {
  cfg.app_data(web::Data::new(AppState {
    app_name: "Actix Test Api".to_string(),
    deployment_env: "dev".to_string(),
  }));
}

fn route_config(cfg: &mut web::ServiceConfig) {
  cfg
    .route("/hi", web::get().to(manual_hello))
    .service(routes::root::get)
    .service(routes::root::post);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new().service(
      web::scope("/api/v1")
        .configure(state_config)
        .configure(route_config),
    )
  })
  .bind(("127.0.0.1", 3000))?
  .run()
  .await
}
