use std::vec::Vec;
use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::models::{User};
use lib::schema::{users, visits, locations};

#[get("/<id>", format = "application/json")]
fn show(conn: DbConn, id: i64) -> Result<Json<User>, ApiError> {
    let user = users::table.find(id).first::<User>(&*conn)?;
    Ok(Json(user))
}

// #[derive(FromForm)]
// struct VisitQuery {
//     fromDate: i64,
//     toDate:   i64,
//     country:  String,
//     toDistance: i64
// }
//
#[derive(Serialize, Queryable)]
struct UserVisits {
    place:      String,
    visited_at: i64,
    mark:       i16
}

#[get("/<id>/visits", format = "application/json")]
fn visits(conn: DbConn, id: i64) -> Result<Json<Vec<UserVisits>>, ApiError> {
    let visits = users::table.inner_join(visits::table.inner_join(locations::table))
        .select((locations::place, visits::visited_at, visits::mark))
        .filter(users::id.eq(id))
        .load(&*conn)?;
    Ok(Json(visits))
}
