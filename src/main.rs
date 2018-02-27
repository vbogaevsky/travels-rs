#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate r2d2_diesel;
extern crate r2d2;

mod lib;

use rocket_contrib::{Json, Value};
use lib::init_pool;
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::models::*;
use diesel::prelude::*;

#[get("/<id>", format = "application/json")]
fn show_user(conn: DbConn, id: i64) -> Result<Json<User>, ApiError> {
    let user = lib::schema::users::table.find(id).first::<User>(&*conn)?;
    Ok(Json(user))
}

// #[post("/", format = "application/json", data = "<params>")]
// fn create_user(params: Json<NewUser>) -> Json<Value> {
//
// }

#[get("/<id>", format = "application/json")]
fn show_location(conn: DbConn, id: i64) -> Result<Json<Location>, ApiError> {
    let location = lib::schema::locations::table.find(id).first::<Location>(&*conn)?;
    Ok(Json(location))
}

#[get("/<id>", format = "application/json")]
fn show_visits(conn: DbConn, id: i64) -> Result<Json<Visit>, ApiError> {
    let visit = lib::schema::visits::table.find(id).first::<Visit>(&*conn)?;
    Ok(Json(visit))
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn main() {
    rocket::ignite()
        .manage(init_pool())
        .mount("/users", routes![show_user])
        // .mount("/users", routes![create_user])
        .mount("/locations", routes![show_location])
        .mount("/visits", routes![show_visits])
        .launch();
}
