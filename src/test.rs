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

fn set_user() -> Result<User, diesel::result::Error> {
    let user = NewUser {
        id: 7004,
        first_name: &String::from("Оксана"),
        last_name:  &String::from("Лебушутева"),
        birth_date: -105321600,
        gender: &String::from("f"),
        email: &String::from("tavonvidel@me.com")
    };
    diesel::insert_into(users::table).values(&user)
                                     .execute(&*DbConn)?
}

fn prep_rocket(route: &str, handler: Vec<rocket::Route>) -> rocket::Rocket {
    rocket::ignite().manage(init_pool())
                    .mount(route, handler)
}

#[test]
fn get_user() {
    let rocketa      = prep_rocket("/users", routes![show_user]);
    let user         = set_user()?;
    let user_json    = serde_json::to_string(&user);
    let client       = Client::new(rocketa).expect("valid rocket instance");
    let mut response = client.get("/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some(user_json));
}
