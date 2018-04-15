use diesel;
use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::schema::users;

#[derive(Deserialize, AsChangeset, Debug)]
#[table_name="users"]
struct UserForm {
    email:      Option<String>,
    first_name: Option<String>,
    last_name:  Option<String>,
    gender:     Option<String>,
    birth_date: Option<i64>
}

#[post("/<id>", format = "application/json", data = "<params>")]
fn update(conn: DbConn, id: i64, params: Json<UserForm>) -> Result<Json<()>, ApiError> {
    let update_data = params.into_inner();
    diesel::update(users::table.find(id)).set(update_data).execute(&*conn)?;
    Ok(Json(()))
}


