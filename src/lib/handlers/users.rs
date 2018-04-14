use std::vec::Vec;
use diesel;
use diesel::prelude::*;
use rocket::request::LenientForm;
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


#[derive(FromForm, AsChangeset, Debug)]
#[table_name="users"]
struct UserForm {
    email:      Option<String>,
    first_name: Option<String>,
    last_name:  Option<String>,
    gender:     Option<String>,
    birth_date: Option<i64>
}

#[post("/<id>", format = "application/json", data = "<params>")]
fn update(conn: DbConn, id: i64, params: LenientForm<UserForm>) -> Result<Json<()>, ApiError> {
    let update_data = params.get();
    diesel::update(users::table.find(id)).set(update_data).execute(&*conn)?;
    Ok(Json(()))
}

#[derive(FromForm, Debug)]
struct VisitParams {
    fromDate:   Option<i64>,
    toDate:     Option<i64>,
    country:    Option<String>,
    toDistance: Option<i64>
}

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
        .filter(users::id.eq(id)).order(visits::visited_at.asc()).load(&*conn)?;
    Ok(Json(visits))
}

#[get("/<id>/visits?<params>", format = "application/json")]
fn queriable_visits(conn: DbConn, id: i64, params: VisitParams) -> Result<Json<Vec<UserVisits>>, ApiError> {
    let mut query = users::table.inner_join(visits::table.inner_join(locations::table))
        .select((locations::place, visits::visited_at, visits::mark))
        .filter(users::id.eq(id)).into_boxed();
    if let Some(fromDate)   = params.fromDate   {
        query = query.filter(visits::visited_at.gt(fromDate));
    }
    if let Some(toDate)     = params.toDate     {
        query = query.filter(visits::visited_at.lt(toDate));
    }
    if let Some(country)    = params.country    {
        query = query.filter(locations::country.eq(country));
    }
    if let Some(toDistance) = params.toDistance {
        query = query.filter(locations::distance.lt(toDistance));
    }
    let visits = query.order(visits::visited_at.asc()).load(&*conn)?;
    Ok(Json(visits))
}

// diesel::sql_types::BigInt
