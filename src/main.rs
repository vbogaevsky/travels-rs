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
                lib::handlers::users::show::show,
                lib::handlers::users::create::create,
                lib::handlers::users::update::update,
                lib::handlers::users::visits::visits,
                lib::handlers::users::visits::queriable_visits
            ]
        ).mount(
            "/locations", routes![
                lib::handlers::locations::show::show,
                lib::handlers::locations::create::create,
                lib::handlers::locations::update::update,
                lib::handlers::locations::avg::avg,
                lib::handlers::locations::avg::queriable_avg
            ]
        ).mount(
            "/visits",    routes![
                lib::handlers::visits::show::show,
                lib::handlers::visits::create::create,
                lib::handlers::visits::update::update
            ]
        ).launch();
}


#[cfg(test)]
pub mod test;
