drop table if exists users;
drop table if exists debit;
drop table if exists credit;
create table users (
    tid integer primary key,
    uname varchar(30) UNIQUE NOT NULL,
    email varchar(30) UNIQUE
);

create table debit (
    id serial primary key,
    debit_amount integer,
    reason varchar(30),
    uid integer,
    foreign key (uid) references users(tid),
    transaction_date varchar(30)
);  

create table credit (
    id serial primary key,
    credit_amount integer,
    reason varchar(30),
    uid integer,
    foreign key (uid) references users(tid),
    transaction_date varchar(30)
);  

