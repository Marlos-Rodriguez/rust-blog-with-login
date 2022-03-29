table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
