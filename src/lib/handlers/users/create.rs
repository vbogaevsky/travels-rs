use diesel;
use diesel::prelude::*;
use rocket_contrib::{Json};
use lib::db_conn::DbConn;
use lib::error::Error as ApiError;
use lib::schema::users;

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
struct UserForm {
   id:         i64,
   email:      String,
   first_name: String,
   last_name:  String,
   gender:     String,
   birth_date: i64
}

#[post("/new", format = "application/json", data = "<params>")]
fn create(conn: DbConn, params: Json<UserForm>) -> Result<Json<()>, ApiError> {
    diesel::insert_into(users::table).values(params.into_inner()).execute(&*conn)?;
    Ok(Json(()))
}
