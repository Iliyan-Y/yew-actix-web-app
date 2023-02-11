use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::controllers::index_controller::get_index;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetIndexParams {
  pub test: String,
}

#[get("/")]
async fn get(req: HttpRequest) -> impl Responder {
  let params = web::Query::<GetIndexParams>::from_query(req.query_string()).unwrap();
  let res = get_index(params);
  HttpResponse::Ok().body(res)
}

#[post("/")]
async fn post(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}
