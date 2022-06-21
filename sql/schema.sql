drop table if exists users;
drop table if exists debit;
create table users (
    tid integer primary key,
    uname varchar(30) UNIQUE NOT NULL
);

create table debit (
    id serial primary key,
    debit_amount integer,
    reason varchar(30),
    uid integer,
    foreign key (uid) references users(tid),
    transaction_date varchar(30)
);  

