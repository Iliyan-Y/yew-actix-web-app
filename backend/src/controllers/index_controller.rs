use actix_web::web::Query;

use crate::routes::index::GetIndexParams;

pub fn get_index(params: Query<GetIndexParams>) -> String {
  format!("HELOO BACKEND {:?}", params.test)
}
