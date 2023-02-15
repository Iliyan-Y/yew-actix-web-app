use crate::{
  entity::{prelude::*, *},
  routes::index::{IndexGetParams, IndexPostBody},
  AppState,
};
use actix_web::{
  web::{self, Data, Json, Query},
  HttpResponse, Responder,
};
use sea_orm::{sea_query::tests_cfg::json, ActiveValue, EntityTrait};
use uuid::Uuid;

pub fn get_index(params: Query<IndexGetParams>, data: Data<AppState>) -> String {
  format!("HELlO BACKEND {:?}, {}", params.test, data.app_name)
}

pub async fn post_user(data: Data<AppState>, body: Json<IndexPostBody>) -> impl Responder {
  let u = user::ActiveModel {
    id: ActiveValue::Set(Uuid::new_v4()),
    email: ActiveValue::Set(body.email.to_owned()),
    password: ActiveValue::Set(body.pass.to_owned()),
    ..Default::default()
  };

  let res = User::insert(u).exec(&data.db).await.unwrap();

  HttpResponse::Ok().body(format!(
    "email is: {}, again without quotes",
    res.last_insert_id
  ))
}

pub async fn get_all_users(data: Data<AppState>) -> impl Responder {
  let users = User::find().all(&data.db).await.unwrap();
  // HttpResponse::Ok().body(format!("{}", json!(users)))
  HttpResponse::Ok().json(users)
}
