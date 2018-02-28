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
    rocket::ignite().manage(init_pool())
                    .mount("/users", routes![show_user])
                    // .mount("/users", routes![create_user])
                    .mount("/locations", routes![show_location])
                    .mount("/visits", routes![show_visits])
                    .launch();
}

#[cfg(test)]
mod test {
    extern crate serde_json;
    extern crate rocket;
    extern crate diesel;
    use std::vec::Vec;
    use rocket::local::Client;
    use rocket::http::Status;
    use diesel::prelude::*;
    use lib::init_pool;
    use lib::db_conn::DbConn;
    use lib::models::{User, NewUser};
    use lib::schema::users;
    use static_rocket_route_info_for_show_user;

    fn set_user() -> User {
        let user = NewUser {
            id: 7004,
            first_name: &String::from("Оксана"),
            last_name:  &String::from("Лебушутева"),
            birth_date: -105321600,
            gender: &String::from("f"),
            email: &String::from("tavonvidel@me.com")
        };

        diesel::insert_into(users::table).values(&user)
                                         .get_result(&DbConn)
                                         .expect("Error saving new user")
    }

    fn prep_rocket(route: &str, handler: Vec<rocket::Route>) -> rocket::Rocket {
        rocket::ignite().manage(init_pool())
                        .mount(route, handler)
    }

    #[test]
    fn get_user() {
        let rocketa      = prep_rocket("/users", routes![show_user]);
        let user         = set_user();
        let user_json    = serde_json::to_string(&user);
        let client       = Client::new(rocketa).expect("valid rocket instance");
        let mut response = client.get("/users/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(user_json));
    }
}
