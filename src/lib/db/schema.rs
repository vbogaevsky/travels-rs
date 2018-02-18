table! {
    locations (internal) {
        internal -> Int4,
        id -> Nullable<Int8>,
        place -> Nullable<Text>,
        country -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        distance -> Nullable<Int8>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    users (internal) {
        internal -> Int4,
        id -> Nullable<Int8>,
        email -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        gender -> Nullable<Bpchar>,
        birth_date -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    visits (internal) {
        internal -> Int4,
        id -> Nullable<Int8>,
        location -> Nullable<Int8>,
        user -> Nullable<Int8>,
        visited_at -> Nullable<Timestamp>,
        mark -> Nullable<Int2>,
        created_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    locations,
    users,
    visits,
);
