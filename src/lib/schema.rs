table! {
    locations (internal) {
        internal -> Int4,
        id -> Nullable<Int8>,
        place -> Nullable<Text>,
        country -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        distance -> Nullable<Int8>,
    }
}

table! {
    users (internal) {
        internal -> Int4,
        id -> Int8,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        gender -> Varchar,
        birth_date -> Int8,
    }
}

table! {
    visits (internal) {
        internal -> Int4,
        id -> Nullable<Int8>,
        location -> Nullable<Int8>,
        user -> Nullable<Int8>,
        visited_at -> Nullable<Int8>,
        mark -> Nullable<Int2>,
    }
}

allow_tables_to_appear_in_same_query!(
    locations,
    users,
    visits,
);
