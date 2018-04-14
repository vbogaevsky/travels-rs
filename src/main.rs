#![feature(plugin)]
#![feature(custom_derive)]
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

#[error(400)]
fn bad_request() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Bad request"
    }))
}

#[error(422)]
fn unprocessable_entity() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Some reason" // TODO: Add reason processing
    }))
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn main() {
    rocket::ignite().manage(init_pool())
        .catch(
            errors![
                bad_request, not_found
            ]
        ).mount(
            "/users",     routes![
                lib::handlers::users::show,
                lib::handlers::users::update,
                lib::handlers::users::visits,
                lib::handlers::users::queriable_visits
            ]
        ).mount(
            "/locations", routes![
                lib::handlers::locations::show,
                lib::handlers::locations::avg,
                lib::handlers::locations::queriable_avg
            ]
        ).mount(
            "/visits",    routes![
                lib::handlers::visits::show
            ]
        ).launch();
}


#[cfg(test)]
pub mod test;
