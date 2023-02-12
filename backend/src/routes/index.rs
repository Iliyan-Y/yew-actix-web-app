use actix_web::{
  get, post,
  web::{self, Json, Query},
  HttpRequest, HttpResponse, Responder,
};

use serde::{Deserialize, Serialize};

use crate::{controllers::index_controller::get_index, AppState};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexGetParams {
  #[serde(default)]
  pub test: String,
}

#[get("/")]
async fn get(
  req: HttpRequest,
  data: web::Data<AppState>,
  index_get_params: Query<IndexGetParams>,
) -> impl Responder {
  let res = get_index(index_get_params, data);
  HttpResponse::Ok().body(res)
}

#[derive(Deserialize, Debug)]
struct IndexPostBody {
  #[serde(default)]
  email: String,
  #[serde(default)]
  pass: i32,
}

#[post("/")]
async fn post(body: Json<IndexPostBody>) -> impl Responder {
  HttpResponse::Ok().body(format!(
    "email is: {}, again without quotes {:?}",
    body.email, body.pass
  ))
}

pub fn index_route_config(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("").service(get).service(post));
}
