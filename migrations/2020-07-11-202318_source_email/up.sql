-- Your SQL goes here
create table source_email (
	id SERIAL PRIMARY KEY,
	domain varchar(100),
	username varchar(100),
	password varchar(100),
	port int,
	connection_mode email_connection_mode,
	auth_mode email_auth_mode,
	display_name varchar(200),
	color varchar(10)
);

create index source_email_display_name on source_email (display_name);
