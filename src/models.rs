#[derive(Queryable, RustcEncodable)]
pub struct User {
    pub internal:   i32,
    pub id:         i64,
    pub email:      String,
    pub first_name: String,
    pub last_name:  String,
    pub gender:     char,
    pub birth_date: i32
}

#[derive(Queryable, RustcEncodable)]
pub struct Location {
    pub internal:   i32,
    pub id:         i64,
    pub place:      String,
    pub country:    String,
    pub city:       String,
    pub distance:   i64
}

#[derive(Queryable, RustcEncodable)]
pub struct Visit {
    pub internal:   i32,
    pub id:         i64,
    pub location:   i64,
    pub user:       i64,
    pub visited_at: i32,
    pub mark:       i16
}
