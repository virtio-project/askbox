-- Add migration script here
drop table if exists ask cascade;
drop table if exists askee cascade;

create table askee (
    id serial not null primary key,
    display_name varchar(255) not null unique,
    created_at timestamp not null default current_timestamp
);

create table ask (
    id serial not null primary key,
    askee integer not null references askee,
    content text not null,
    dedup varchar(255) not null unique,
    created_at timestamp not null default current_timestamp
);

create index on askee(display_name);
create index on ask(askee);