drop table if exists users;
drop table if exists debit;
create table users (
    uid serial primary key,
    name varchar(30)
);

create table debit (
    id serial primary key,
    debit_amount integer,
    reason varchar(30),
    uid integer,
    foreign key (uid) references users(uid),
    transaction_date varchar(30)
);  

