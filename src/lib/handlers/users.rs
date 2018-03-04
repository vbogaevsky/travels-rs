use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::models::{User};
use lib::schema::users;

#[get("/<id>", format = "application/json")]
fn show(conn: DbConn, id: i64) -> Result<Json<User>, ApiError> {
    let user = users::table.find(id).first::<User>(&*conn)?;
    Ok(Json(user))
}
