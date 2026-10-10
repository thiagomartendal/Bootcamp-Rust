create database if not exists clientes_rust;

use clientes_rust;

create table if not exists cliente (
    id int auto_increment primary key,
    nome varchar(150),
    telefone varchar(20)
);

insert into cliente (nome, telefone) values ('Cliente 1', '1234-5678'), ('Cliente 2', '8765-4321');
