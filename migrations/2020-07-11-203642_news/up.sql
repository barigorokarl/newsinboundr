-- Your SQL goes here
create table news (
	id SERIAL PRIMARY KEY,
	guid varchar(100),
	source_type news_source_type,
	source_id int,
	insert_date timestamp,
	title varchar(200),
	content text,
	link varchar(500),
	color varchar(10)
);

create index news_source on news (source_type, source_id);
create unique index news_guid on news (guid);
