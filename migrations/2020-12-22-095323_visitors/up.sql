create table if not exists visitors
(
    visitor_id  serial       not null primary key,
    prison_id varchar(11)   not null,
    gender  varchar(1)   not null, -- M = Male, F = Female
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    relations VARCHAR(100) NOT NULL,
    phone_num VARCHAR(50) NOT NULL,
    line_id VARCHAR(30) NOT NULL,
    remark   VARCHAR(200) NOT NULL,
    created_at   timestamp    not null,
    updated_at   timestamp
);
create index visitor_by_first_name_last_name on visitors(first_name, last_name);