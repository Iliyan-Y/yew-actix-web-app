use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

struct AppState {
  app_name: String,
  deployment_env: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Params {
  test: String,
}

#[get("/")]
async fn hello(req: HttpRequest) -> impl Responder {
  let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
  let res = format!("HELOO BACKEND {:?}", params.test);
  HttpResponse::Ok().body(res)
}

#[post("/boo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

async fn manual_hello(data: web::Data<AppState>) -> impl Responder {
  let res = format!("HELOO from {}", &data.app_name);
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
    .service(hello)
    .service(echo);
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
