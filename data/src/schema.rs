table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        refresh_token -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        uuid -> Bpchar,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
