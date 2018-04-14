use diesel;
use diesel::prelude::*;
use rocket::request::LenientForm;
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

#[derive(FromForm, AsChangeset, Debug)]
#[table_name = "visits"]
struct VisitForm {
    location:   Option<i64>,
    user:       Option<i64>,
    visited_at: Option<i64>,
    mark:       Option<i16>
}

#[post("/<id>", format = "application/json", data = "<params>")]
fn update(conn: DbConn, id: i64, params: LenientForm<VisitForm>) -> Result<Json<()>, ApiError> {
    let update_data = params.get();
    diesel::update(visits::table.find(id)).set(update_data).execute(&*conn)?;
    Ok(Json(()))
} 
