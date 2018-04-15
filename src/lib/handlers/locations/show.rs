use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::models::{Location};
use lib::schema::locations;

#[get("/<id>", format = "application/json")]
fn show(conn: DbConn, id: i64) -> Result<Json<Location>, ApiError> {
    let location = locations::table.find(id).first::<Location>(&*conn)?;
    Ok(Json(location))
}


