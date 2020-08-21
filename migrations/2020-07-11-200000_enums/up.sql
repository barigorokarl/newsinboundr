-- Your SQL goes here
create type email_connection_mode as enum ('insecure', 'SSL/TLS', 'STARTTLS');
create type email_auth_mode as enum('password plain', 'password crypted');
create type news_source_type as enum('rss', 'atom', 'email');
