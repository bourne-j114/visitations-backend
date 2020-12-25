table! {
    prisons (prison_id) {
        prison_id -> Varchar,
        gender -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        location -> Varchar,
        case_detail -> Varchar,
        punish -> Varchar,
        remark -> Varchar,
        id_card -> Varchar,
        jail_date -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

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
    visitors (visitor_id) {
        visitor_id -> Int4,
        prison_id -> Varchar,
        gender -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        relations -> Varchar,
        phone_num -> Varchar,
        line_id -> Varchar,
        remark -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    visits (id) {
        id -> Uuid,
        prison_id -> Varchar,
        visitor_id -> Int4,
        visit_date -> Timestamp,
        start_time -> Timestamp,
        stop_time -> Timestamp,
        round -> Int2,
        allow -> Int2,
        remark -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    prisons,
    users,
    visitors,
    visits,
);
