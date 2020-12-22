create table if not exists visitors
(
    id           serial       not null primary key,
    name         varchar(100) not null,
    address1     varchar(150) not null,
    address2     varchar(150) not null,
    post_code    varchar(50)  not null,
    id_number    varchar(13)  not null,
    gender       varchar(1)   not null, -- M = Male, F = Female
    phone_number varchar(50)  not null,
    created_at   timestamp    not null,
    updated_at   timestamp
);