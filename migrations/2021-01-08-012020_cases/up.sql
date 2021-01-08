-- Your SQL goes here
create table if not exists cases
(
    id  UUID  not null primary key,
    prison_id VARCHAR(11) NOT NULL,
    court_order VARCHAR(100) NOT NULL,
    case_no VARCHAR(20) NOT NULL,
    case_detail VARCHAR(500) NOT NULL,
    police_station VARCHAR(100) NOT NULL,
    catch_date date    not null,
    receive_date date    not null,
    jail_date   date    not null,
    jail_status   smallint NOT NULL DEFAULT 0,
    scheduled_release15 date ,
    scheduled_release45 date,
    cause_release VARCHAR(100) NOT NULL,
    created_at   timestamp    not null,
    updated_at   timestamp
    );
create index cases_by_receive_date on cases(receive_date);
create index cases_by_prison_id on cases(prison_id);
create index cases_by_case_no on cases(case_no);
