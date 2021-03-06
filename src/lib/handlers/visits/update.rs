use diesel;
use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::schema::visits;

#[derive(Deserialize, AsChangeset, Debug)]
#[table_name = "visits"]
struct VisitForm {
    location:   Option<i64>,
    user:       Option<i64>,
    visited_at: Option<i64>,
    mark:       Option<i16>
}

#[post("/<id>", format = "application/json", data = "<params>")]
fn update(conn: DbConn, id: i64, params: Json<VisitForm>) -> Result<Json<()>, ApiError> {
    let update_data = params.into_inner();
    diesel::update(visits::table.find(id)).set(update_data).execute(&*conn)?;
    Ok(Json(()))
}
