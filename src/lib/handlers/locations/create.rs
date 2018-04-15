use diesel;
use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::schema::locations;

#[derive(Deserialize, Insertable)]
#[table_name = "locations"]
struct LocationForm {
    id:       i64,
    place:    String,
    country:  String,
    city:     String,
    distance: i64
}

#[post("/new", format = "application/json", data = "<params>")]
fn create(conn: DbConn, params: Json<LocationForm>) -> Result<Json<()>, ApiError> {
    diesel::insert_into(locations::table).values(params.into_inner()).execute(&*conn)?;
    Ok(Json(()))
}
