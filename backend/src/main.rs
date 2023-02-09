use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

struct AppState {
  app_name: String,
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(
        web::scope("/api/v1")
          .app_data(web::Data::new(AppState {
            app_name: "Actix Test Api".to_string(),
          }))
          .service(echo)
          .route("/hey", web::get().to(manual_hello)),
      )
      .service(hello)
  })
  .bind(("127.0.0.1", 3000))?
  .run()
  .await
}
