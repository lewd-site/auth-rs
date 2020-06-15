table! {
    users (id) {
        id -> Int4,
        uuid -> Bpchar,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        refresh_token -> Nullable<Varchar>,
    }
}
