use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Params {
  test: String,
}

#[get("/")]
async fn get(req: HttpRequest) -> impl Responder {
  let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
  let res = format!("HELOO BACKEND {:?}", params.test);
  HttpResponse::Ok().body(res)
}

#[post("/")]
async fn post(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}
