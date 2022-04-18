table! {
    posts (id) {
        id -> Varchar,
        title -> Varchar,
        slug -> Varchar,
        author_id -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
