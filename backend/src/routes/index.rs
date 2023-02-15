use actix_web::{
  get, post,
  web::{self, Json, Query},
  HttpRequest, HttpResponse, Responder,
};

use serde::{Deserialize, Serialize};

use crate::{
  controllers::index_controller::{get_index, post_user},
  AppState,
};

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
pub struct IndexPostBody {
  #[serde(default)]
  pub email: String,
  #[serde(default)]
  pub pass: String,
}

#[post("/")]
async fn post(body: Json<IndexPostBody>, data: web::Data<AppState>) -> impl Responder {
  post_user(data, body).await
}

pub fn index_route_config(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("").service(get).service(post));
}
