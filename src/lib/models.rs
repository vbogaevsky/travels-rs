use super::schema::{users, locations, visits};
#[derive(Serialize, Queryable)]
pub struct User {
    pub internal:       i32,
    pub id:             i64,
    pub email:          String,
    pub first_name:     String,
    pub last_name:      String,
    pub gender:         char,
    pub birth_date:     i32
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser<'t> {
    pub id:             i64,
    pub email:      &'t str,
    pub first_name: &'t str,
    pub last_name:  &'t str,
    pub gender:     &'t str,
    pub birth_date:     i64
}

#[derive(Queryable, Serialize)]
pub struct Location {
    pub internal:       i32,
    pub id:             i64,
    pub place:          String,
    pub country:        String,
    pub city:           String,
    pub distance:       i64
}

#[derive(Deserialize, Insertable)]
#[table_name = "locations"]
pub struct NewLocation<'t> {
    pub id:             i64,
    pub place:      &'t str,
    pub country:    &'t str,
    pub city:       &'t str,
    pub distance:       i64
}

#[derive(Queryable, Serialize)]
pub struct Visit {
    pub internal:       i32,
    pub id:             i64,
    pub location:       i64,
    pub user:           i64,
    pub visited_at:     i32,
    pub mark:           i16
}

#[derive(Deserialize, Insertable)]
#[table_name = "visits"]
pub struct NewVisit {
    pub id:             i64,
    pub location:       i64,
    pub user:           i64,
    pub visited_at:     i64,
    pub mark:           i16
}
