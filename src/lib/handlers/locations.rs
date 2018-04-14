use std::time::{SystemTime, UNIX_EPOCH};
use diesel;
use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::models::{Location, Visit};
use lib::schema::{locations, visits, users};

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

#[get("/<id>/avg?<params>", format = "application/json")]
fn queriable_avg(conn: DbConn, id: i64, params: AvgParams) -> Result<Json<AvgMark>, ApiError> {
    let mut query = locations::table.inner_join(visits::table.inner_join(users::table))
        .select(visits::mark).filter(locations::id.eq(id))
        .into_boxed();
    if let Some(fromDate) = params.fromDate {
        query = query.filter(visits::visited_at.gt(fromDate));
    }
    if let Some(toDate)   = params.toDate   {
        query = query.filter(visits::visited_at.lt(toDate));
    }
    if let Some(fromAge)  = params.fromAge  {
        query = query.filter(users::birth_date.lt(current_time() - fromAge));
    }
    if let Some(toAge)    = params.toAge    {
        query = query.filter(users::birth_date.gt(current_time() - toAge));
    }
    if let Some(gender)   = params.gender   {
        query = query.filter(users::gender.eq(gender));
    }
    let marks: Vec<i16> = query.load(&*conn)?;
    let sum: f64 = marks.iter().fold(0.0, |sum, &mark| sum + mark as f64);
    let totalVisits: f64 = marks.len() as f64;
    let avg = sum / totalVisits;
    Ok(Json(AvgMark{ avg: avg }))
}

fn current_time() -> i64 {
    let duration_since = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration_since.as_secs() as i64
}
