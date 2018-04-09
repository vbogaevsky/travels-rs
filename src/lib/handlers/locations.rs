use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::models::{Location, Visit};
use lib::schema::locations;

#[derive(FromForm, Debug)]
struct AvgParams {
    fromDate: Option<i64>,
    toDate:   Option<i64>,
    fromAge:  Option<i64>,
    toAge:    Option<i64>,
    gender:   Option<String>
}

#[derive(Serialize, Queryable)]
struct AvgMark {
    avg: f64
}

#[get("/<id>", format = "application/json")]
fn show(conn: DbConn, id: i64) -> Result<Json<Location>, ApiError> {
    let location = locations::table.find(id).first::<Location>(&*conn)?;
    Ok(Json(location))
}

#[get("/<id>/avg", format = "application/json")]
fn avg(conn: DbConn, id: i64) -> Result<Json<AvgMark>, ApiError> {
    let location = locations::table.find(id).first::<Location>(&*conn)?;
    let visits = Visit::belonging_to(&location)
        .load::<Visit>(&*conn)?;
    let sum: f64 = visits.iter().map (|v| v.mark)
        .fold(0.0, |sum, mark| sum + mark as f64);
    let totalVisits: f64 = visits.len() as f64;
    let avg = sum / totalVisits;
    Ok(Json(AvgMark{ avg: avg }))
}

//#[get("/<id>/avg?<params>", format = "application/json")]
//fn queriable_avg(conn: DbConn, id: i64, params: AvgParams) -> Result<Json<AvgMark>, ApiError> {
//}

