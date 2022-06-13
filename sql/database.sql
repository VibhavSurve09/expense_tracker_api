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
    foreign key (uid) references users(uid)
);

insert into  users  (name) values ('Vibhav'),('X');
insert into debit (debit_amount,reason) values (500,'Drinks'), (600,'Clubs');