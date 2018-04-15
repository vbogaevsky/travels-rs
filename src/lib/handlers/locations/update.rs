use diesel;
use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::schema::locations;

#[derive(Deserialize, AsChangeset, Debug)]
#[table_name = "locations"]
struct LocationForm {
    place:    Option<String>,
    country:  Option<String>,
    city:     Option<String>,
    distance: Option<i64>
}

#[post("/<id>", format = "application/json", data = "<params>")]
fn update(conn: DbConn, id: i64, params: Json<LocationForm>) -> Result<Json<()>, ApiError> {
    let update_data = params.into_inner();
    diesel::update(locations::table.find(id)).set(update_data).execute(&*conn)?;
    Ok(Json(()))
}


