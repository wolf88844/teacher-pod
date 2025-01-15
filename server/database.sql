create table if not exists auth(
    id varchar(128) not null primary key,
    account int not null,
    expire int not null
);

create table if not exists account(
    id  serial not null primary key,
    email varchar(255) default '' unique,
    username varchar(255) not null,
    gender varchar(10) default 'unknown',
    reg_date date default current_date,
    recently date default current_date,
    password varchar(255) not null,
    salt varchar(20) not null,
    introduction text default '',
    avatar varchar(255) default '/assets/image/default_avatar.png',
    role int default 0
);
--test data
insert into account (email, username, password, salt, avatar) values ('mrxzx@qq.com', 'mrxiaozhuox', '22b48f7a98d9a8d684c7000dde01ef6e', 'ccIvwLYPegqy', 'https://avatars.githubusercontent.com/u/41265098?v=4');


create table if not exists config(
    id serial not null primary key,
    name varchar(100) not null,
    value text not null
);

--- default config data
insert into config(name,value) values('RecommendList','[]');

create table if not exists activity(
    id serial not null primary key,
    account int not null,
    type int default 0,
    target int not null,
    curr_time timestamp default current_timestamp
);

create table if not exists favorite(
    id serial not null primary key,
    account int not null,
    podcast varchar(50) not null
);