use actix_web::web::{Data, Query};

use crate::{routes::index::IndexGetParams, AppState};

pub fn get_index(params: Query<IndexGetParams>, data: Data<AppState>) -> String {
  format!("HELlO BACKEND {:?}, {}", params.test, data.app_name)
}
