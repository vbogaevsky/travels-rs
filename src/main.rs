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
use lib::models::*;
use diesel::prelude::*;

#[get("/<id>", format = "application/json")]
fn show_user(conn: DbConn, id: i64) -> Result<Json<User>, diesel::result::Error> {
    use lib::schema::users::dsl;
    let result = users.find(id).first::<User>(conn);
    match result.first().ok_or(diesel::result::Error::NotFound) {
        Ok(user) => Ok(Json(*user)),
        Err(e)     => Err(e)
    }
}

// #[post("/", format = "application/json", data = "<params>")]
// fn create_user(params: Json<NewUser>) -> Json<Value> {
//
// }

// #[get("/<id>", format = "application/json")]
// fn show_location(id: i64) -> String {
//     format!("Location #{}", id)
// }
//
// #[get("/<id>", format = "application/json")]
// fn show_visits(id: i64) -> String {
//     format!("Visit #{}", id)
// }

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
        // .mount("/locations", routes![show_location])
        // .mount("/visits", routes![show_visits])
        .launch();
}
