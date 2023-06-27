-- Add migration script here
create table company (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "name" varchar(120) not null
);

create table country (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "name" varchar(100) not null
);

insert into country (name) values ('United States');

create table prog_language (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "name" varchar(100) not null
);

insert into prog_language (name) values ('Rust');
insert into prog_language (name) values ('Go');
insert into prog_language (name) values ('Ruby');
insert into prog_language (name) values ('Swift');
insert into prog_language (name) values ('Kotlin');
insert into prog_language (name) values ('Scala');
insert into prog_language (name) values ('Elixir');

create table industry (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "name" varchar(100) not null
);

insert into industry (name) values ('Finance');
insert into industry (name) values ('Crypto/Blockchain');
insert into industry (name) values ('AI/ML');
insert into industry (name) values ('Games');

create table salary (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "base" int not null
);

insert into salary (base) values (200000);
insert into salary (base) values (300000);
insert into salary (base) values (400000);
insert into salary (base) values (500000);

create table developer (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "user_name" varchar(50) not null,
    "full_name" varchar(100) not null,
    "primary_lang_id" bigserial not null,

    constraint fk_primary_lang foreign key(primary_lang_id) references prog_language(id)
);

create table employer (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,    
    "user_name" varchar(50) not null,
    "full_name" varchar(100) not null,
    "email" varchar(120) not null unique,
    "company_id" bigserial not null,

    constraint fk_company foreign key(company_id) references company(id)
);

create table job (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "employer_id" bigserial not null,
    "title" varchar(60) not null,
    "description" varchar(600) not null,
    "is_remote" boolean not null,
    "headquarters_country_id" bigserial not null,
    "primary_lang_id" bigserial not null,
    "secondary_lang_id" bigserial,
    "industry_id" bigserial not null,
    "salary_id" bigserial not null,

    constraint fk_employer foreign key(employer_id) references employer(id),
    constraint fk_headquarters_country foreign key(headquarters_country_id) references country(id),
    constraint fk_primary_lang foreign key(primary_lang_id) references prog_language(id),
    constraint fk_secondary_lang foreign key(secondary_lang_id) references prog_language(id),
    constraint fk_industry foreign key(industry_id) references industry(id),
    constraint fk_salary foreign key(salary_id) references salary(id)
);