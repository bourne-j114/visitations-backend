table! {
    cases (id) {
        id -> Uuid,
        prison_id -> Varchar,
        court_order -> Varchar,
        case_no -> Varchar,
        case_detail -> Varchar,
        police_station -> Varchar,
        catch_date -> Date,
        receive_date -> Date,
        jail_date -> Date,
        jail_status -> Int2,
        scheduled_release15 -> Nullable<Date>,
        scheduled_release45 -> Nullable<Date>,
        cause_release -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    prisons (prison_id) {
        prison_id -> Varchar,
        gender -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        nick_name -> Varchar,
        birth_day -> Varchar,
        picture_paht -> Varchar,
        location -> Varchar,
        prison_type -> Int2,
        remark -> Varchar,
        id_card -> Varchar,
        address_no -> Varchar,
        moo -> Varchar,
        subdistric -> Varchar,
        distric -> Varchar,
        province -> Varchar,
        race -> Varchar,
        nationality -> Varchar,
        religion -> Varchar,
        blame -> Varchar,
        education -> Varchar,
        edu_institution -> Varchar,
        status -> Varchar,
        child -> Int2,
        sibling -> Int2,
        child_in_a_child -> Int2,
        home_owner -> Varchar,
        stay_address_no -> Varchar,
        stay_moo -> Varchar,
        stay_subdistric -> Varchar,
        stay_distric -> Varchar,
        stay_province -> Varchar,
        occupation -> Varchar,
        income -> Varchar,
        history_punish -> Varchar,
        history_punish_year -> Int2,
        history_punish_month -> Int2,
        history_punish_day -> Int2,
        prove_pass_num -> Int2,
        cur_num -> Int2,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        userid -> Text,
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
        visitor_name -> Varchar,
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
    cases,
    prisons,
    users,
    visitors,
    visits,
);
