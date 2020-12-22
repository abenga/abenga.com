
use crate::lib::db::models as db_models;
use crate::lib::db::utils as db_utils;

pub fn posts() /*-> db_models::Post*/ {
    let conn = db_utils::get_connection().expect("Failure of database connection.");
}
