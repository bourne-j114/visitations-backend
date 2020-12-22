table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        login_session -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    visitors (id) {
        id -> Int4,
        name -> Varchar,
        address1 -> Varchar,
        address2 -> Varchar,
        post_code -> Varchar,
        id_number -> Varchar,
        gender -> Varchar,
        phone_number -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    visitors,
);
