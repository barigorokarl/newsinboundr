-- Your SQL goes here
create table source_rss (
	id SERIAL PRIMARY KEY,
	url text,
	display_name varchar(200),
	color varchar(10)
);

create index source_rss_display_name on source_rss (display_name);
