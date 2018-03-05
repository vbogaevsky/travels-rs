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

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn main() {
    rocket::ignite().manage(init_pool()).catch(errors![not_found])
        .mount(
            "/users",     routes![
                lib::handlers::users::show,
                lib::handlers::users::visits
            ]
        ).mount(
            "/locations", routes![
                lib::handlers::locations::show
            ]
        ).mount(
            "/visits",    routes![
                lib::handlers::visits::show
            ]
        ).launch();
}


#[cfg(test)]
pub mod test;
