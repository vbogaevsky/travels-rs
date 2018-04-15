use diesel;
use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::schema::visits;

#[derive(Deserialize, Insertable)]
#[table_name = "visits"]
struct VisitForm {
    id:         i64,
    location:   i64,
    user:       i64,
    visited_at: i64,
    mark:       i16
}

#[post("/new", format = "application/json", data = "<params>")]
fn create(conn: DbConn, params: Json<VisitForm>) -> Result<Json<()>, ApiError> {
    diesel::insert_into(visits::table).values(params.into_inner()).execute(&*conn)?;
    Ok(Json(()))
}
