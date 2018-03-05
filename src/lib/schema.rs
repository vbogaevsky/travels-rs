table! {
    locations (id) {
        id         -> Int8,
        place      -> Text,
        country    -> Varchar,
        city       -> Varchar,
        distance   -> Int8,
    }
}

table! {
    users (id) {
        id         -> Int8,
        email      -> Varchar,
        first_name -> Varchar,
        last_name  -> Varchar,
        gender     -> Varchar,
        birth_date -> Int8,
    }
}

table! {
    visits (id) {
        id         -> Int8,
        location   -> Int8,
        user       -> Int8,
        visited_at -> Int8,
        mark       -> Int2,
    }
}

joinable!(visits   -> users(user));
joinable!(visits   -> locations(location));

allow_tables_to_appear_in_same_query!(locations, users, visits);
