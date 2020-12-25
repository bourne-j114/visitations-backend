-- Your SQL goes here
create table if not exists visits
(
    id  UUID       not null primary key,
    prison_id  VARCHAR(11) NOT NULL,
    visitor_id  integer NOT NULL,
    visit_date timestamp not null,
    start_time timestamp not null,
    stop_time timestamp not null,
    round smallint NOT NULL,
    allow smallint NOT NULL,
    remark   VARCHAR(200) NOT NULL,
    created_at   timestamp    not null,
    updated_at   timestamp
    );