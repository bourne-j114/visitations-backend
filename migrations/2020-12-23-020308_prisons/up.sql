-- Your SQL goes here
create table if not exists prisons
(
    prison_id  VARCHAR(11) NOT NULL primary key,
    gender  varchar(1)   not null, -- M = Male, F = Female
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    location VARCHAR(50) NOT NULL,
    prison_type smallint NOT NULL DEFAULT 0,
    case_detail VARCHAR(500) NOT NULL,
    punish VARCHAR(200) NOT NULL,
    remark   VARCHAR(200) NOT NULL,
    id_card   VARCHAR(20) NOT NULL,
    jail_date   VARCHAR(10) NOT NULL,
    created_at   timestamp    not null,
    updated_at   timestamp
    );
create index prisons_by_first_name_last_name on prisons(first_name, last_name);