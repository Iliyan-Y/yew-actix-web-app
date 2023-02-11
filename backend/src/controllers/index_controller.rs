use actix_web::web::{Data, Query};

use crate::{routes::index::GetIndexParams, AppState};

pub fn get_index(params: Query<GetIndexParams>, data: Data<AppState>) -> String {
  format!("HELOO BACKEND {:?}, {}", params.test, data.app_name)
}
