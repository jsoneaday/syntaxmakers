CREATE EXTENSION pgcrypto;

create table country (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "name" varchar(100) not null
);

create table company (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "name" varchar(120) not null unique,
    "logo" bytea,
    "headquarters_country_id" bigserial not null,

    constraint fk_headquarters_country_id foreign key(headquarters_country_id) references country(id)
);

create table prog_language (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "name" varchar(100) not null
);

create table industry (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "name" varchar(100) not null
);

create table salary (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "base" int not null
);

create table developer (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "user_name" varchar(60) not null,
    "full_name" varchar(100) not null,
    "email" varchar(120) not null unique,
    "password" varchar(200) not null,
    "description" varchar(5000) not null,
    "primary_lang_id" bigserial not null,

    constraint fk_primary_lang foreign key(primary_lang_id) references prog_language(id)
);

create table developers_secondary_langs (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "developer_id" bigserial not null,
    "secondary_lang_id" bigserial not null
);

create table employer (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,        
    "user_name" varchar(60) not null,
    "full_name" varchar(100) not null,
    "email" varchar(120) not null unique,
    "password" varchar(200) not null,
    "company_id" bigserial not null,

    constraint fk_company foreign key(company_id) references company(id)
);

create table job (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "employer_id" bigserial not null,
    "title" varchar(100) not null,
    "description" varchar(8000) not null,
    "is_remote" boolean not null,
    "primary_lang_id" bigserial not null,
    "secondary_lang_id" bigserial,
    "industry_id" bigserial not null,
    "salary_id" bigserial not null,

    constraint fk_employer foreign key(employer_id) references employer(id),
    constraint fk_primary_lang foreign key(primary_lang_id) references prog_language(id),
    constraint fk_secondary_lang foreign key(secondary_lang_id) references prog_language(id),
    constraint fk_industry foreign key(industry_id) references industry(id),
    constraint fk_salary foreign key(salary_id) references salary(id)
);

create table jobs_countries (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp,
    "job_id" bigserial not null,
    "country_id" bigserial not null,

    constraint fk_job foreign key(job_id) references job(id),
    constraint fk_country foreign key(country_id) references country(id)
);

create table application (
    "id" bigserial primary key,
    "created_at" timestamptz(3) not null default current_timestamp,
    "updated_at" timestamptz(3) not null default current_timestamp, 
    "job_id" bigserial not null,
    "developer_id" bigserial not null,

    constraint fk_job foreign key(job_id) references job(id),
    constraint fk_developer foreign key(developer_id) references developer(id)
);