use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::models::{Visit};
use lib::schema::visits;

#[get("/<id>", format = "application/json")]
fn show(conn: DbConn, id: i64) -> Result<Json<Visit>, ApiError> {
    let visit = visits::table.find(id).first::<Visit>(&*conn)?;
    Ok(Json(visit))
}
